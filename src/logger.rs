use anyhow::Result;
use serde_json;
use std::fs::OpenOptions;
use std::io::Write;
use log::info;

use crate::arbitrage::ArbitrageOpportunity;

pub struct OpportunityLogger {
    log_file: String,
}

impl OpportunityLogger {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            log_file: "arbitrage_opportunities.jsonl".to_string(),
        })
    }
    
    pub async fn log_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
        let log_entry = serde_json::json!({
            "id": opportunity.id,
            "timestamp": opportunity.timestamp.to_rfc3339(),
            "buy_exchange": opportunity.buy_exchange,
            "sell_exchange": opportunity.sell_exchange,
            "buy_price": opportunity.buy_price,
            "sell_price": opportunity.sell_price,
            "estimated_profit": opportunity.estimated_profit,
            "price_difference": opportunity.price_difference,
            "gas_cost_estimate": opportunity.gas_cost_estimate,
            "profit_percentage": opportunity.profit_percentage
        });
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;
            
        writeln!(file, "{}", log_entry)?;
        
        info!("Opportunity logged to file: {}", self.log_file);
        
        Ok(())
    }
}
