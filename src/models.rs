use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ArbitrageOpportunity {
    pub id: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub estimated_profit: f64,
    pub profit_percentage: f64,
    pub created_at: String,
}
