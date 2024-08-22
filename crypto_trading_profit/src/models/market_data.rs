use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub last_price: f64,
    pub volume: f64,
    pub price_change: f64,
    pub price_change_percent: f64,
}