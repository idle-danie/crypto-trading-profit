use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;
use tokio::task; 

pub async fn get_upbit_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let base_url = env::var("UPBIT_API_URL").expect("UPBIT_API_URL 환경 변수를 설정하세요.");
    let url = format!("{}?markets={}", base_url, symbol);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await?;

    let upbit_data: serde_json::Value = response.json().await?;
    let symbol_owned = symbol.to_string(); 

    // CPU 집약적인 작업을 spawn_blocking을 통해 별도의 스레드 풀로 위임
    let market_data_result = task::spawn_blocking(move || {
        if let Some(data) = upbit_data.as_array().and_then(|arr| arr.get(0)) {
            let market_data = MarketData {
                symbol: normalize_symbol(&symbol_owned),
                exchange_name: "Upbit".to_string(),
                opening_price: parse_f64(&data["opening_price"]),
                high_price: parse_f64(&data["high_price"]),
                low_price: parse_f64(&data["low_price"]),
                last_price: parse_f64(&data["trade_price"]),
                price_change: parse_f64(&data["signed_change_price"]),
                price_change_percent: parse_f64(&data["signed_change_rate"]) * 100.0,
                volume: parse_f64(&data["acc_trade_volume_24h"]),
            };
            Some(market_data)
        } else {
            None
        }
    }).await;
    
    // spawn_blocking의 결과를 처리
    match market_data_result {
        Ok(Some(market_data)) => Ok(market_data), // 성공적으로 파싱된 경우
        Ok(None) => { // 파싱은 성공했으나 데이터 구조가 예상과 다른 경우
            eprintln!("Upbit API 응답 형식이 예상과 다릅니다.");
            Err(reqwest::Error::from(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid API response format")
            ))
        },
        Err(e) => { // Blocking task 자체가 실패한 경우 (패닉 등)
            eprintln!("Blocking task panicked: {}", e);
            Err(reqwest::Error::from(
                std::io::Error::new(std::io::ErrorKind::Other, "Blocking task failed")
            ))
        }
    }
}