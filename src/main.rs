mod config;
mod dex;
mod price_fetcher;
mod arbitrage;
mod logger;
mod db;
mod api;

use std::time::Duration;
use tokio::time;
use anyhow::Result;
use log::warn;
use clap::Parser;
use actix_web::{web, App, HttpServer, middleware::Logger as ActixLogger};
use actix_cors::Cors;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::price_fetcher::PriceFetcher;
use crate::arbitrage::ArbitrageDetector;
use crate::logger::OpportunityLogger;
use crate::db::Database;

#[derive(Parser)]
#[command(name = "polygon-arbitrage-bot")]
#[command(about = "A Polygon arbitrage opportunity detection bot")]
struct Cli {
    #[arg(short, long, default_value = "config/config.toml")]
    config: String,
    
    #[arg(short, long)]
    debug: bool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let log_level = if cli.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();
    
    println!("Starting Polygon Arbitrage Bot");
    
    let config = AppConfig::load(&cli.config).await?;
    let config = Arc::new(config);
    
    println!("Configuration loaded successfully");
    println!("Monitoring WETH/USDC pair on QuickSwap vs SushiSwap");
    println!("Minimum profit threshold: {:.2} USDC", 
          config.trading.min_profit_usdc.parse::<f64>().unwrap() / 1_000_000.0);
    
    let database = Arc::new(Database::new(&config.database.url).await?);
    println!("Database initialized");
    
    let price_fetcher = Arc::new(PriceFetcher::new(&config).await?);
    let detector = Arc::new(ArbitrageDetector::new(&config));
    let logger = Arc::new(OpportunityLogger::new().await?);
    
    println!("Bot components initialized successfully");
    
    // Clone references for the async task
    let db_clone = database.clone();
    let fetcher_clone = price_fetcher.clone();
    let detector_clone = detector.clone();
    let logger_clone = logger.clone();
    let config_clone = config.clone();
    
    tokio::spawn(async move {
        monitor_arbitrage_loop(db_clone, fetcher_clone, detector_clone, logger_clone, config_clone).await;
    });
    
    // CRITICAL FIX: Clone database for the HTTP server
    let db_for_server = database.clone();
    
    println!("HTTP API server starting on http://127.0.0.1:8081");
    println!("Starting monitoring loop (interval: {}s)", config.monitoring.check_interval_seconds);
    println!("-----------------------------------------------------");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Accept", "Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(ActixLogger::default())
            .app_data(web::Data::new(db_for_server.clone())) // Use the cloned database
            .service(api::get_opportunities)
            .service(api::get_stats)
            .service(api::get_status)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await?;

    Ok(())
}

async fn monitor_arbitrage_loop(
    database: Arc<Database>,
    fetcher: Arc<PriceFetcher>,
    detector: Arc<ArbitrageDetector>,
    logger: Arc<OpportunityLogger>,
    config: Arc<AppConfig>,
) {
    let mut interval = time::interval(Duration::from_secs(config.monitoring.check_interval_seconds));
    let mut iteration_count = 0u64;
    
    loop {
        interval.tick().await;
        iteration_count += 1;
        
        print!("Check #{}: Fetching prices from DEXes... ", iteration_count);
        
        match run_arbitrage_check(&database, &fetcher, &detector, &logger).await {
            Ok(found_opportunity) => {
                if !found_opportunity {
                    println!("No profitable arbitrage opportunities found");
                }
            },
            Err(e) => {
                println!("Error in arbitrage check: {}", e);
                warn!("Continuing with next iteration...");
            }
        }
    }
}

async fn run_arbitrage_check(
    database: &Database,
    fetcher: &PriceFetcher,
    detector: &ArbitrageDetector,
    logger: &OpportunityLogger,
) -> Result<bool> {
    let (quickswap_result, sushiswap_result) = tokio::join!(
        fetcher.get_quickswap_price(),
        fetcher.get_sushiswap_price()
    );
    
    let quickswap_price = quickswap_result?;
    let sushiswap_price = sushiswap_result?;
    
    print!("QuickSwap={:.6} USDC, SushiSwap={:.6} USDC -> ", quickswap_price, sushiswap_price);
    
    let price_diff = (quickswap_price - sushiswap_price).abs();
    
    if let Some(opportunity) = detector.detect_opportunity(quickswap_price, sushiswap_price) {
        println!("ARBITRAGE OPPORTUNITY DETECTED!");
        println!("   Buy on: {} at {:.6} USDC", opportunity.buy_exchange, opportunity.buy_price);
        println!("   Sell on: {} at {:.6} USDC", opportunity.sell_exchange, opportunity.sell_price);
        println!("   Estimated profit: {:.6} USDC", opportunity.estimated_profit);
        println!("   Price difference: {:.6} USDC", opportunity.sell_price - opportunity.buy_price);
        
        logger.log_opportunity(&opportunity).await?;
        database.store_opportunity(&opportunity).await?;
        
        return Ok(true);
    } else {
        println!("Price diff: {:.6} USDC (below threshold)", price_diff);
    }
    
    Ok(false)
}
