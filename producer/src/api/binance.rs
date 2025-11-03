use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;
use tokio::task; 

pub async fn get_binance_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let url = env::var("BINANCE_API_URL").expect("BINANCE_API_URL 환경 변수를 설정하세요.");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[("symbol", symbol)])
        .send()
        .await?;
        
    let binance_data: serde_json::Value = response.json().await?;
    let symbol_owned = symbol.to_string();

    // CPU 집약적인 작업을 spawn_blocking을 통해 별도의 스레드 풀로 위임
    let market_data_result = task::spawn_blocking(move || {
        MarketData {
            symbol: normalize_symbol(&symbol_owned),
            exchange_name: "Binance".to_string(),
            opening_price: parse_f64(&binance_data["openPrice"]),
            high_price: parse_f64(&binance_data["highPrice"]),
            low_price: parse_f64(&binance_data["lowPrice"]),
            last_price: parse_f64(&binance_data["lastPrice"]),
            price_change: parse_f64(&binance_data["priceChange"]),
            price_change_percent: parse_f64(&binance_data["priceChangePercent"]),
            volume: parse_f64(&binance_data["volume"]),
        }
    }).await;
    
    // spawn_blocking은 JoinError를 반환할 수 있으므로 처리
    match market_data_result {
        Ok(market_data) => Ok(market_data),
        Err(e) => {
            eprintln!("Blocking task panicked: {}", e);
            // reqwest::Error와 호환되는 에러를 반환
            Err(reqwest::Error::from(
                std::io::Error::new(std::io::ErrorKind::Other, "Blocking task failed")
            ))
        }
    }
}