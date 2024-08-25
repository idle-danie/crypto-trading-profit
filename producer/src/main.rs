mod utils;
mod api;
mod models;
mod kafka;
mod services;  

use tokio::time::{sleep, Duration};
use rdkafka::producer::FutureProducer;
use models::crypto_symbols::CryptoSymbols;
use services::service::fetch_and_send;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = kafka::producer::create_producer();

    let btc_symbols = CryptoSymbols::new("BTCUSDT", "BTC-USDT", "USDT-BTC", "KRW-BTC");
    let eth_symbols = CryptoSymbols::new("ETHUSDT", "ETH-USDT", "USDT-ETH", "KRW-ETH");

    loop {
        // BTC 데이터 가져오기 및 전송
        fetch_and_send(&producer, &btc_symbols, "BTC").await;

        // ETH 데이터 가져오기 및 전송
        fetch_and_send(&producer, &eth_symbols, "ETH").await;

        // 5초 동안 대기
        sleep(Duration::from_secs(5)).await;
    }
}