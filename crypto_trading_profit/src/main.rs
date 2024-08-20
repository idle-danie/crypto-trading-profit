mod utils; 
mod api;
mod models;

#[tokio::main]
async fn main() {
    let binance_symbol = "BTCUSDT";
    let kucoin_symbol = "BTC-USDT";
    let upbit_symbol = "USDT-BTC";

    match api::binance::get_binance_ohlcv(binance_symbol).await {
        Ok(market_data) => println!("Binance: {:?}", market_data),
        Err(e) => eprintln!("Failed to fetch Binance data: {}", e),
    }

    match api::kucoin::get_kucoin_ohlcv(kucoin_symbol).await {
        Ok(market_data) => println!("KuCoin: {:?}", market_data),
        Err(e) => eprintln!("Failed to fetch KuCoin data: {}", e),
    }

    match api::upbit::get_upbit_ohlcv(upbit_symbol).await {
        Ok(market_data) => println!("Upbit: {:?}", market_data),
        Err(e) => eprintln!("Failed to fetch Upbit data: {}", e),
    }

    match api::bithumb::get_bithumb_ohlcv().await {
        Ok(market_data) => println!("Bithumb: {:?}", market_data),
        Err(e) => eprintln!("Failed to fetch Bithumb data: {}", e),
    }
}