use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::parse_f64;  

pub async fn get_upbit_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let url = format!("https://api.upbit.com/v1/ticker?markets={}", symbol);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .send()
        .await?;

    let upbit_data: serde_json::Value = response.json().await?;
    let upbit_data = &upbit_data[0];  

    let market_data = MarketData {
        opening_price: parse_f64(&upbit_data["opening_price"]),
        high_price: parse_f64(&upbit_data["high_price"]),
        low_price: parse_f64(&upbit_data["low_price"]),
        last_price: parse_f64(&upbit_data["trade_price"]),
        price_change: parse_f64(&upbit_data["signed_change_price"]),
        price_change_percent: parse_f64(&upbit_data["signed_change_rate"]) * 100.0,
        volume: parse_f64(&upbit_data["acc_trade_volume_24h"]),
    };

    Ok(market_data)
}