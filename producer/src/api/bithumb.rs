use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;
use tokio::task; 

pub async fn get_bithumb_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let base_url = env::var("BITHUMB_API_URL").expect("BITHUMB_API_URL 환경 변수를 설정하세요.");
    let url = format!("{}?markets={}", base_url, symbol);
    let krw_usdt_url = format!("{}?markets=KRW-USDT", base_url);
    let client = reqwest::Client::new();
    let symbol_owned = symbol.to_string(); 

    let response = client.get(&url).send().await?;
    let krw_usdt_response = client.get(&krw_usdt_url).send().await?;
    
    let bithumb_data: serde_json::Value = response.json().await?;
    let krw_usdt_data: serde_json::Value = krw_usdt_response.json().await?;

    // CPU 집약적인 작업을 spawn_blocking을 통해 별도의 스레드 풀로 위임
    let market_data_result = task::spawn_blocking(move || {
        let krw_crypto_price = parse_f64(&bithumb_data[0]["trade_price"]);
        let krw_usdt_price = parse_f64(&krw_usdt_data[0]["trade_price"]);
        let crypto_usdt_price = krw_crypto_price / krw_usdt_price;

        MarketData {
            symbol: normalize_symbol(&symbol_owned),
            exchange_name: "Bithumb".to_string(),
            opening_price: parse_f64(&bithumb_data[0]["opening_price"]) / krw_usdt_price,
            high_price: parse_f64(&bithumb_data[0]["high_price"]) / krw_usdt_price,
            low_price: parse_f64(&bithumb_data[0]["low_price"]) / krw_usdt_price,
            last_price: crypto_usdt_price,
            price_change: parse_f64(&bithumb_data[0]["signed_change_price"]) / krw_usdt_price,
            price_change_percent: parse_f64(&bithumb_data[0]["signed_change_rate"]) * 100.0,
            volume: parse_f64(&bithumb_data[0]["acc_trade_volume_24h"]),
        }
    }).await;
    
    // spawn_blocking은 JoinError를 반환할 수 있으므로 처리
    match market_data_result {
        Ok(market_data) => Ok(market_data),
        Err(e) => {
            // JoinError는 보통 패닉이 발생했을 때 나타나므로, 에러 로깅 후 변환
            eprintln!("Blocking task panicked: {}", e);
            Err(reqwest::Error::from(
                std::io::Error::new(std::io::ErrorKind::Other, "Blocking task failed")
            ))
        }
    }
}