pub mod uniswap;
pub mod sushiswap;

use anyhow::Result;
use ethers::types::U256;
use async_trait::async_trait;

#[async_trait]
pub trait DexPriceFetcher: Send + Sync {
    async fn get_price(&self, amount_in: U256) -> Result<U256>;
    fn get_name(&self) -> &str;
}

pub use uniswap::QuickSwapPriceFetcher;
pub use sushiswap::SushiSwapPriceFetcher;
