use serde::Deserialize;
use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub dex: DexConfig,
    pub tokens: TokenConfig,
    pub trading: TradingConfig,
    pub monitoring: MonitoringConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    pub rpc_url: String,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DexConfig {
    pub quickswap_router: String,
    pub sushiswap_router: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TokenConfig {
    pub weth: String,
    pub usdc: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradingConfig {
    pub trade_amount_wei: String,
    pub min_profit_usdc: String,
    pub gas_estimate_usdc: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub check_interval_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl AppConfig {
    pub async fn load(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path))?;
        
        let config: AppConfig = toml::from_str(&config_content)
            .with_context(|| "Failed to parse config file")?;
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    fn validate(&self) -> Result<()> {
        // Validate addresses format (basic check)
        if !self.dex.quickswap_router.starts_with("0x") {
            anyhow::bail!("Invalid QuickSwap router address format");
        }
        if !self.dex.sushiswap_router.starts_with("0x") {
            anyhow::bail!("Invalid SushiSwap router address format");
        }
        if !self.tokens.weth.starts_with("0x") {
            anyhow::bail!("Invalid WETH token address format");
        }
        if !self.tokens.usdc.starts_with("0x") {
            anyhow::bail!("Invalid USDC token address format");
        }
        
        // Validate numeric values
        self.trading.trade_amount_wei.parse::<u64>()
            .with_context(|| "Invalid trade_amount_wei format")?;
        self.trading.min_profit_usdc.parse::<u64>()
            .with_context(|| "Invalid min_profit_usdc format")?;
        self.trading.gas_estimate_usdc.parse::<u64>()
            .with_context(|| "Invalid gas_estimate_usdc format")?;
        
        Ok(())
    }
}
