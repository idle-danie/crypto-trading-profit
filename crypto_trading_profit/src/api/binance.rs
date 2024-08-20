use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::parse_f64;  

pub async fn get_binance_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let url = "https://api.binance.com/api/v3/ticker/24hr";
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .query(&[("symbol", symbol)])
        .send()
        .await?;

    let binance_data: serde_json::Value = response.json().await?;

    let market_data = MarketData {
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