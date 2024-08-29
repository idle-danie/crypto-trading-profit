use reqwest::Error;
use crate::models::market_data::MarketData;
use crate::utils::{normalize_symbol, parse_f64};
use std::env;

pub async fn get_bithumb_ohlcv(symbol: &str) -> Result<MarketData, Error> {
    let base_url = env::var("BITHUMB_API_URL").expect("BITHUMB_API_URL 환경 변수를 설정하세요.");

    // {symbol}-USDT에 대한 API가 없어, KRW-{symbol} 및 KRW-USDT의 데이터를 통해 환산
    let url = format!("{}?markets={}", base_url, symbol);
    let krw_usdt_url = format!("{}?markets=KRW-USDT", base_url);

    let client = reqwest::Client::new();

    // KRW-{symbol}와 KRW-USDT 데이터를 비동기적으로 가져옴
    let response = client.get(&url).send().await?;
    let krw_usdt_response = client.get(&krw_usdt_url).send().await?;

    let bithumb_data: serde_json::Value = response.json().await?;
    let krw_usdt_data: serde_json::Value = krw_usdt_response.json().await?;

    // KRW-{symbol}와 KRW-USDT의 가격을 추출
    let krw_crypto_price = parse_f64(&bithumb_data[0]["trade_price"]);
    let krw_usdt_price = parse_f64(&krw_usdt_data[0]["trade_price"]);

    // {symbol}-USDT 가격을 계산
    let crypto_usdt_price = krw_crypto_price / krw_usdt_price;

    let market_data = MarketData {
        symbol: normalize_symbol(symbol).to_string(),
        exchange_name: "Bithumb".to_string(),
        opening_price: parse_f64(&bithumb_data[0]["opening_price"]) / krw_usdt_price,
        high_price: parse_f64(&bithumb_data[0]["high_price"]) / krw_usdt_price,
        low_price: parse_f64(&bithumb_data[0]["low_price"]) / krw_usdt_price,
        last_price: crypto_usdt_price,
        price_change: parse_f64(&bithumb_data[0]["signed_change_price"]) / krw_usdt_price,
        price_change_percent: parse_f64(&bithumb_data[0]["signed_change_rate"]) * 100.0,
        volume: parse_f64(&bithumb_data[0]["acc_trade_volume_24h"]),
    };

    Ok(market_data)
}
