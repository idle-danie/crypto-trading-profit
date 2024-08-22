use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::parse_f64;  

pub async fn get_kucoin_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let url = format!("https://api.kucoin.com/api/v1/market/stats?symbol={}", symbol);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .send()
        .await?;

    let kucoin_data: serde_json::Value = response.json().await?;

    let last_price = parse_f64(&kucoin_data["data"]["last"]);
    let change_price = parse_f64(&kucoin_data["data"]["changePrice"]);
    let opening_price = last_price - change_price;

    let market_data = MarketData {
        opening_price,
        high_price: parse_f64(&kucoin_data["data"]["high"]),
        low_price: parse_f64(&kucoin_data["data"]["low"]),
        last_price,
        volume: parse_f64(&kucoin_data["data"]["vol"]),
        price_change: change_price,
        price_change_percent: parse_f64(&kucoin_data["data"]["changeRate"]) * 100.0,
    };

    Ok(market_data)
}