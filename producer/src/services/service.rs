use crate::api::{binance, kucoin, upbit, bithumb};
use crate::utils;
use rdkafka::producer::FutureProducer;
use serde_json;
use crate::models::crypto_symbols::CryptoSymbols;
use crate::kafka;

pub async fn fetch_and_send(
    producer: &FutureProducer,
    symbols: &CryptoSymbols<'_>,
    crypto_name: &str,
) {
    // Binance
    match binance::get_binance_ohlcv(symbols.binance).await {
        Ok(market_data) => {
            println!("{} Binance: {:?}", crypto_name, market_data);
            let payload = serde_json::to_string(&market_data).unwrap();
            kafka::producer::send_to_kafka(
                producer,
                "binance-topic",
                utils::normalize_symbol(symbols.binance),
                payload.as_bytes(),
            )
            .await;
        }
        Err(e) => eprintln!("{} Binance 데이터 가져오기 실패: {}", crypto_name, e),
    }

    // KuCoin
    match kucoin::get_kucoin_ohlcv(symbols.kucoin).await {
        Ok(market_data) => {
            println!("{} KuCoin: {:?}", crypto_name, market_data);
            let payload = serde_json::to_string(&market_data).unwrap();
            kafka::producer::send_to_kafka(
                producer,
                "kucoin-topic",
                utils::normalize_symbol(symbols.kucoin),
                payload.as_bytes(),
            )
            .await;
        }
        Err(e) => eprintln!("{} KuCoin 데이터 가져오기 실패: {}", crypto_name, e),
    }

    // Upbit
    match upbit::get_upbit_ohlcv(symbols.upbit).await {
        Ok(market_data) => {
            println!("{} Upbit: {:?}", crypto_name, market_data);
            let payload = serde_json::to_string(&market_data).unwrap();
            kafka::producer::send_to_kafka(
                producer,
                "upbit-topic",
                utils::normalize_symbol(symbols.upbit),
                payload.as_bytes(),
            )
            .await;
        }
        Err(e) => eprintln!("{} Upbit 데이터 가져오기 실패: {}", crypto_name, e),
    }

    // Bithumb
    match bithumb::get_bithumb_ohlcv(symbols.bithumb).await {
        Ok(market_data) => {
            println!("{} Bithumb: {:?}", crypto_name, market_data);
            let payload = serde_json::to_string(&market_data).unwrap();
            kafka::producer::send_to_kafka(
                producer,
                "bithumb-topic",
                utils::normalize_symbol(symbols.bithumb),
                payload.as_bytes(),
            )
            .await;
        }
        Err(e) => eprintln!("{} Bithumb 데이터 가져오기 실패: {}", crypto_name, e),
    }
}