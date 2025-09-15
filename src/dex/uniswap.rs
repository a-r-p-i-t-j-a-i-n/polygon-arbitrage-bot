use super::DexPriceFetcher;
use anyhow::{Result, Context};
use ethers::{
    contract::abigen,
    providers::{Provider, Http},
    types::{Address, U256},
};
use std::{str::FromStr, sync::Arc};
use async_trait::async_trait;
use log::debug;

abigen!(
    IUniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts)
    ]"#
);

pub struct QuickSwapPriceFetcher {
    router: IUniswapV2Router<Provider<Http>>,
    weth_address: Address,
    usdc_address: Address,
}

impl QuickSwapPriceFetcher {
    pub fn new(
        provider: Arc<Provider<Http>>,
        router_address: &str,
        weth_address: &str,
        usdc_address: &str,
    ) -> Result<Self> {
        let router_addr = Address::from_str(router_address)
            .with_context(|| format!("Invalid QuickSwap router address: {}", router_address))?;
        let router = IUniswapV2Router::new(router_addr, provider);
        
        Ok(Self {
            router,
            weth_address: Address::from_str(weth_address)
                .with_context(|| format!("Invalid WETH address: {}", weth_address))?,
            usdc_address: Address::from_str(usdc_address)
                .with_context(|| format!("Invalid USDC address: {}", usdc_address))?,
        })
    }
}

#[async_trait]
impl DexPriceFetcher for QuickSwapPriceFetcher {
    async fn get_price(&self, amount_in: U256) -> Result<U256> {
        debug!("Fetching price from QuickSwap for {} WETH", 
               ethers::utils::format_ether(amount_in));
        
        let path = vec![self.weth_address, self.usdc_address];
        let amounts = self.router
            .get_amounts_out(amount_in, path)
            .call()
            .await
            .with_context(|| "Failed to call getAmountsOut on QuickSwap router")?;
        
        if amounts.len() != 2 {
            anyhow::bail!("Invalid response from QuickSwap router: expected 2 amounts, got {}", amounts.len());
        }
        
        debug!("QuickSwap price response: {} USDC", amounts[1]);
        Ok(amounts[1])
    }
    
    fn get_name(&self) -> &str {
        "QuickSwap"
    }
}
