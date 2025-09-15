use crate::config::AppConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub estimated_profit: f64,
    pub price_difference: f64,
    pub gas_cost_estimate: f64,
    pub profit_percentage: f64,
}

pub struct ArbitrageDetector {
    min_profit_threshold: f64,
    gas_cost_estimate: f64,
}

impl ArbitrageDetector {
    pub fn new(config: &AppConfig) -> Self {
        let min_profit = config.trading.min_profit_usdc.parse::<u64>().unwrap() as f64 / 1_000_000.0;
        let gas_cost = config.trading.gas_estimate_usdc.parse::<u64>().unwrap() as f64 / 1_000_000.0;
        
        debug!("Arbitrage detector initialized:");
        debug!("  Min profit threshold: {:.6} USDC", min_profit);
        debug!("  Gas cost estimate: {:.6} USDC", gas_cost);
        
        Self {
            min_profit_threshold: min_profit,
            gas_cost_estimate: gas_cost,
        }
    }
    
    pub fn detect_opportunity(
        &self,
        quickswap_price: f64,
        sushiswap_price: f64,
    ) -> Option<ArbitrageOpportunity> {
        let price_difference = (quickswap_price - sushiswap_price).abs();
        let estimated_profit = price_difference - self.gas_cost_estimate;
        
        debug!("Arbitrage analysis:");
        debug!("  QuickSwap price: {:.6} USDC", quickswap_price);
        debug!("  SushiSwap price: {:.6} USDC", sushiswap_price);
        debug!("  Price difference: {:.6} USDC", price_difference);
        debug!("  Gas cost estimate: {:.6} USDC", self.gas_cost_estimate);
        debug!("  Estimated profit: {:.6} USDC", estimated_profit);
        debug!("  Min profit threshold: {:.6} USDC", self.min_profit_threshold);
        
        if estimated_profit > self.min_profit_threshold {
            let (buy_exchange, sell_exchange, buy_price, sell_price) = 
                if quickswap_price < sushiswap_price {
                    ("QuickSwap".to_string(), "SushiSwap".to_string(), quickswap_price, sushiswap_price)
                } else {
                    ("SushiSwap".to_string(), "QuickSwap".to_string(), sushiswap_price, quickswap_price)
                };
            
            let profit_percentage = (estimated_profit / buy_price) * 100.0;
            
            Some(ArbitrageOpportunity {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                buy_exchange,
                sell_exchange,
                buy_price,
                sell_price,
                estimated_profit,
                price_difference,
                gas_cost_estimate: self.gas_cost_estimate,
                profit_percentage,
            })
        } else {
            debug!("No profitable opportunity found (profit {} < threshold {})",
                   estimated_profit, self.min_profit_threshold);
            None
        }
    }
}
