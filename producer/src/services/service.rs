use crate::api::{binance, kucoin, upbit, bithumb};
use rdkafka::producer::FutureProducer;
use serde_json;
use crate::models::crypto_symbols::CryptoSymbols;
use crate::kafka;
use tokio::join; 

pub async fn fetch_and_send(
    producer: &FutureProducer,
    symbols: &CryptoSymbols<'_>,
    crypto_name: &str,
) {
    // 4개의 비동기 작업을 바로 await하지 않고 Future로 생성
    let binance_future = binance::get_binance_ohlcv(symbols.binance);
    let kucoin_future = kucoin::get_kucoin_ohlcv(symbols.kucoin);
    let upbit_future = upbit::get_upbit_ohlcv(symbols.upbit);
    let bithumb_future = bithumb::get_bithumb_ohlcv(symbols.bithumb);

    // join! 매크로를 사용하여 4개의 Future를 동시에 실행
    let (binance_res, kucoin_res, upbit_res, bithumb_res) = 
        join!(binance_future, kucoin_future, upbit_future, bithumb_future);

    // 각 결과를 처리
    if let Ok(market_data) = binance_res {
        let payload = serde_json::to_string(&market_data).unwrap();
        kafka::producer::send_to_kafka(producer, "binance-topic", &market_data.symbol, payload.as_bytes()).await;
    } else if let Err(e) = binance_res {
        eprintln!("{} Binance 데이터 가져오기 실패: {}", crypto_name, e);
    }

    if let Ok(market_data) = kucoin_res {
        let payload = serde_json::to_string(&market_data).unwrap();
        kafka::producer::send_to_kafka(producer, "kucoin-topic", &market_data.symbol, payload.as_bytes()).await;
    } else if let Err(e) = kucoin_res {
        eprintln!("{} KuCoin 데이터 가져오기 실패: {}", crypto_name, e);
    }
    
    if let Ok(market_data) = upbit_res {
        let payload = serde_json::to_string(&market_data).unwrap();
        kafka::producer::send_to_kafka(producer, "upbit-topic", &market_data.symbol, payload.as_bytes()).await;
    } else if let Err(e) = upbit_res {
        eprintln!("{} Upbit 데이터 가져오기 실패: {}", crypto_name, e);
    }
    
    if let Ok(market_data) = bithumb_res {
        let payload = serde_json::to_string(&market_data).unwrap();
        kafka::producer::send_to_kafka(producer, "bithumb-topic", &market_data.symbol, payload.as_bytes()).await;
    } else if let Err(e) = bithumb_res {
        eprintln!("{} Bithumb 데이터 가져오기 실패: {}", crypto_name, e);
    }
}