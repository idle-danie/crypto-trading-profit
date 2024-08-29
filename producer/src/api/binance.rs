use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;

pub async fn get_binance_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let url = env::var("BINANCE_API_URL").expect("BINANCE_API_URL 환경 변수를 설정하세요.");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[("symbol", symbol)])
        .send()
        .await?;
        
    let binance_data: serde_json::Value = response.json().await?;

    let market_data = MarketData {
        symbol: normalize_symbol(symbol).to_string(),
        exchange_name: "Binance".to_string(),
        opening_price: parse_f64(&binance_data["openPrice"]),
        high_price: parse_f64(&binance_data["highPrice"]),
        low_price: parse_f64(&binance_data["lowPrice"]),
        last_price: parse_f64(&binance_data["lastPrice"]),
        price_change: parse_f64(&binance_data["priceChange"]),
        price_change_percent: parse_f64(&binance_data["priceChangePercent"]),
        volume: parse_f64(&binance_data["volume"]),
    };

    Ok(market_data)
}
