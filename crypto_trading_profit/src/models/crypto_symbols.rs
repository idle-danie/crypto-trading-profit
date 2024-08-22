pub struct CryptoSymbols<'a> {
    pub binance: &'a str,
    pub kucoin: &'a str,
    pub upbit: &'a str,
    pub bithumb: &'a str,
}

impl<'a> CryptoSymbols<'a> {
    pub fn new(binance: &'a str, kucoin: &'a str, upbit: &'a str, bithumb: &'a str) -> Self {
        Self {
            binance,
            kucoin,
            upbit,
            bithumb,
        }
    }
}