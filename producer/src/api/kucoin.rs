use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;
use tokio::task; 

pub async fn get_kucoin_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let base_url = env::var("KUCOIN_API_URL").expect("KUCOIN_API_URL 환경 변수를 설정하세요.");
    let url = format!("{}?symbol={}", base_url, symbol);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await?;

    let kucoin_data: serde_json::Value = response.json().await?;
    let symbol_owned = symbol.to_string(); 

    // CPU 집약적인 작업을 spawn_blocking을 통해 별도의 스레드 풀로 위임
    let market_data_result = task::spawn_blocking(move || {
        let last_price = parse_f64(&kucoin_data["data"]["last"]);
        let change_price = parse_f64(&kucoin_data["data"]["changePrice"]);
        let opening_price = last_price - change_price;

        MarketData {
            symbol: normalize_symbol(&symbol_owned),
            exchange_name: "Kucoin".to_string(),
            opening_price,
            high_price: parse_f64(&kucoin_data["data"]["high"]),
            low_price: parse_f64(&kucoin_data["data"]["low"]),
            last_price,
            volume: parse_f64(&kucoin_data["data"]["vol"]),
            price_change: change_price,
            price_change_percent: parse_f64(&kucoin_data["data"]["changeRate"]) * 100.0,
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