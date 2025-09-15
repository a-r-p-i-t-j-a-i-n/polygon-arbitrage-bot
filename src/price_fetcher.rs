use anyhow::{Result, Context};
use ethers::{
    providers::{Provider, Http}, 
    types::U256,
    middleware::Middleware,
};
use std::{str::FromStr, sync::Arc};
use log::info;

use crate::config::AppConfig;
use crate::dex::{
    QuickSwapPriceFetcher,
    SushiSwapPriceFetcher,
    DexPriceFetcher,
};

pub struct PriceFetcher {
    quickswap: QuickSwapPriceFetcher,
    sushiswap: SushiSwapPriceFetcher,
    trade_amount: U256,
}

impl PriceFetcher {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        info!("Connecting to Polygon RPC: {}", config.network.rpc_url);
        
        let provider = Provider::<Http>::try_from(&config.network.rpc_url)
            .with_context(|| format!("Failed to connect to RPC: {}", config.network.rpc_url))?;
        let provider = Arc::new(provider);
        
        let chain_id = provider.get_chainid().await
            .with_context(|| "Failed to get chain ID from provider")?;
        
        if chain_id.as_u64() != config.network.chain_id {
            anyhow::bail!("Chain ID mismatch: expected {}, got {}", 
                         config.network.chain_id, chain_id);
        }
        
        info!("Connected to Polygon (Chain ID: {})", chain_id);
        
        let quickswap = QuickSwapPriceFetcher::new(
            provider.clone(),
            &config.dex.quickswap_router,
            &config.tokens.weth,
            &config.tokens.usdc,
        ).with_context(|| "Failed to initialize QuickSwap price fetcher")?;
        
        let sushiswap = SushiSwapPriceFetcher::new(
            provider,
            &config.dex.sushiswap_router,
            &config.tokens.weth,
            &config.tokens.usdc,
        ).with_context(|| "Failed to initialize SushiSwap price fetcher")?;
        
        let trade_amount = U256::from_str(&config.trading.trade_amount_wei)
            .with_context(|| "Invalid trade amount format")?;
        
        info!("Trade amount: {} WETH", ethers::utils::format_ether(trade_amount));
        
        Ok(Self {
            quickswap,
            sushiswap,
            trade_amount,
        })
    }
    
    pub async fn get_quickswap_price(&self) -> Result<f64> {
        let usdc_amount = self.quickswap.get_price(self.trade_amount).await?;
        Ok(usdc_to_float(usdc_amount))
    }
    
    pub async fn get_sushiswap_price(&self) -> Result<f64> {
        let usdc_amount = self.sushiswap.get_price(self.trade_amount).await?;
        Ok(usdc_to_float(usdc_amount))
    }
}

fn usdc_to_float(usdc_amount: U256) -> f64 {
    // USDC has 6 decimals
    usdc_amount.as_u128() as f64 / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_usdc_to_float() {
        let usdc_amount = U256::from(5_000_000u64); // 5 USDC
        assert_eq!(usdc_to_float(usdc_amount), 5.0);
        
        let usdc_amount = U256::from(1_500_000u64); // 1.5 USDC
        assert_eq!(usdc_to_float(usdc_amount), 1.5);
    }
}
