use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;

pub async fn get_kucoin_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let base_url = env::var("KUCOIN_API_URL").expect("KUCOIN_API_URL 환경 변수를 설정하세요.");
    let url = format!("{}?symbol={}", base_url, symbol);
    
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
        symbol: normalize_symbol(symbol).to_string(),
        exchange_name: "Kucoin".to_string(),
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
