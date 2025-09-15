use anyhow::{Result, Context};
use sqlx::{SqlitePool, Row, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use log::{info, debug, error};

use crate::arbitrage::ArbitrageOpportunity;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Initializing database connection: {}", database_url);
        
        let options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true);
        
        let pool = SqlitePool::connect_with(options).await
            .with_context(|| format!("Failed to connect to database: {}", database_url))?;
        
        let db = Self { pool };
        
        db.create_tables().await?;
        
        info!("Database initialized successfully");
        
        Ok(db)
    }
    
    async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                buy_exchange TEXT NOT NULL,
                sell_exchange TEXT NOT NULL,
                buy_price REAL NOT NULL,
                sell_price REAL NOT NULL,
                estimated_profit REAL NOT NULL,
                price_difference REAL NOT NULL,
                gas_cost_estimate REAL NOT NULL,
                profit_percentage REAL NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await
        .with_context(|| "Failed to create arbitrage_opportunities table")?;
        
        debug!("Database tables created/verified");
        
        Ok(())
    }
    
    pub async fn store_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
        info!("Storing opportunity with ID: {}", opportunity.id);
        
        sqlx::query(
            r#"
            INSERT INTO arbitrage_opportunities 
            (id, timestamp, buy_exchange, sell_exchange, buy_price, sell_price, 
             estimated_profit, price_difference, gas_cost_estimate, profit_percentage)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&opportunity.id)
        .bind(opportunity.timestamp.to_rfc3339())
        .bind(&opportunity.buy_exchange)
        .bind(&opportunity.sell_exchange)
        .bind(opportunity.buy_price)
        .bind(opportunity.sell_price)
        .bind(opportunity.estimated_profit)
        .bind(opportunity.price_difference)
        .bind(opportunity.gas_cost_estimate)
        .bind(opportunity.profit_percentage)
        .execute(&self.pool)
        .await
        .with_context(|| "Failed to store arbitrage opportunity")?;
        
        info!("Opportunity stored in database with ID: {}", opportunity.id);
        
        Ok(())
    }

    pub async fn get_recent_opportunities(&self, limit: i64) -> Result<Vec<ArbitrageOpportunity>> {
        info!("Querying database for {} recent opportunities", limit);
        
        let rows = sqlx::query(
            "SELECT * FROM arbitrage_opportunities ORDER BY created_at DESC LIMIT ?1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .with_context(|| "Failed to fetch recent opportunities")?;
        
        info!("Found {} rows in database", rows.len());
        
        let mut opportunities = Vec::new();
        
        for row in rows {
            match Self::row_to_opportunity(&row) {
                Ok(opportunity) => opportunities.push(opportunity),
                Err(e) => {
                    error!("Failed to parse row: {}", e);
                    continue;
                }
            }
        }
        
        info!("Successfully parsed {} opportunities", opportunities.len());
        Ok(opportunities)
    }

    fn row_to_opportunity(row: &sqlx::sqlite::SqliteRow) -> Result<ArbitrageOpportunity> {
        let timestamp_str: String = row.get("timestamp");
        let timestamp = timestamp_str.parse()
            .with_context(|| format!("Failed to parse timestamp: {}", timestamp_str))?;
        
        Ok(ArbitrageOpportunity {
            id: row.get("id"),
            timestamp,
            buy_exchange: row.get("buy_exchange"),
            sell_exchange: row.get("sell_exchange"),
            buy_price: row.get("buy_price"),
            sell_price: row.get("sell_price"),
            estimated_profit: row.get("estimated_profit"),
            price_difference: row.get("price_difference"),
            gas_cost_estimate: row.get("gas_cost_estimate"),
            profit_percentage: row.get("profit_percentage"),
        })
    }

    pub async fn get_stats(&self) -> Result<(i64, f64, f64)> {
        info!("Querying database stats");
        
        let row = sqlx::query_as::<_, (i64, Option<f64>, Option<f64>)>(
            r#"
            SELECT 
                COUNT(*) as total,
                AVG(estimated_profit) as avg_profit,
                MAX(estimated_profit) as max_profit
            FROM arbitrage_opportunities
            "#
        )
        .fetch_one(&self.pool)
        .await
        .with_context(|| "Failed to fetch stats")?;

        let stats = (row.0, row.1.unwrap_or(0.0), row.2.unwrap_or(0.0));
        info!("Stats: total={}, avg={:.2}, max={:.2}", stats.0, stats.1, stats.2);
        
        Ok(stats)
    }
}
