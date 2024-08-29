from pyspark.sql import functions as F
from pyspark.sql import DataFrame
from processing.fees import Fees


def process_kafka_stream(df: DataFrame) -> DataFrame:
    # 기본적인 데이터 처리: Kafka에서 들어온 데이터를 'crypto_prices' 테이블 형식으로 변환
    df = df.withColumnRenamed("last_price", "price").withColumn("timestamp", F.current_timestamp())  # 수집 시점 추가
    return df


def calculate_arbitrage_average(df: DataFrame) -> DataFrame:
    foreign_exchanges = ["Binance", "Kucoin"]
    korean_exchanges = ["Upbit", "Bithumb"]

    # 워터마크를 설정하여 30초 동안의 지연 데이터 허용 (타임스탬프 기준)
    df_with_watermark = df.withWatermark("timestamp", "30 seconds")

    # 외국 거래소 평균 가격 계산 (3분 윈도우 적용)
    foreign_avg = (
        df_with_watermark.filter(df.exchange_name.isin(foreign_exchanges))
        .groupBy("symbol", F.window("timestamp", "3 minutes").alias("window_foreign"))
        .agg(F.avg("price").alias("foreign_avg"))
    )

    # 국내 거래소 평균 가격 계산 (3분 윈도우 적용)
    korean_avg = (
        df_with_watermark.filter(df.exchange_name.isin(korean_exchanges))
        .groupBy("symbol", F.window("timestamp", "3 minutes").alias("window_korean"))
        .agg(F.avg("price").alias("korean_avg"))
    )

    # 매매 차익 비율 계산
    arbitrage_avg_df = (
        foreign_avg.join(korean_avg, ["symbol"])
        .withColumn("profit_percent", (F.col("korean_avg") - F.col("foreign_avg")) / F.col("foreign_avg") * 100)
        .withColumn("timestamp", F.col("window_foreign.start"))  # foreign window의 start를 timestamp로 사용
        .select("timestamp", "foreign_avg", "korean_avg", "profit_percent", "symbol")
    )

    return arbitrage_avg_df


def calculate_arbitrage_values(df: DataFrame) -> DataFrame:
    # Binance -> Upbit
    binance_upbit_btc = (
        (F.col("price") * Fees.BINANCE_BTC_TRADE_FEE - Fees.BINANCE_BTC_WITHDRAW_FEE) * Fees.UPBIT_BTC_TRADE_FEE
    ) - F.col("price")
    binance_upbit_eth = (
        (F.col("price") * Fees.BINANCE_ETH_TRADE_FEE - Fees.BINANCE_ETH_WITHDRAW_FEE) * Fees.UPBIT_ETH_TRADE_FEE
    ) - F.col("price")

    # Binance -> Bithumb
    binance_bithumb_btc = (
        (F.col("price") * Fees.BINANCE_BTC_TRADE_FEE - Fees.BINANCE_BTC_WITHDRAW_FEE) * Fees.BITHUMB_BTC_TRADE_FEE
    ) - F.col("price")
    binance_bithumb_eth = (
        (F.col("price") * Fees.BINANCE_ETH_TRADE_FEE - Fees.BINANCE_ETH_WITHDRAW_FEE) * Fees.BITHUMB_ETH_TRADE_FEE
    ) - F.col("price")

    # KuCoin -> Upbit
    kucoin_upbit_btc = (
        (F.col("price") * Fees.KUCOIN_BTC_TRADE_FEE - Fees.KUCOIN_BTC_WITHDRAW_FEE) * Fees.UPBIT_BTC_TRADE_FEE
    ) - F.col("price")
    kucoin_upbit_eth = (
        (F.col("price") * Fees.KUCOIN_ETH_TRADE_FEE - Fees.KUCOIN_ETH_WITHDRAW_FEE) * Fees.UPBIT_ETH_TRADE_FEE
    ) - F.col("price")

    # KuCoin -> Bithumb
    kucoin_bithumb_btc = (
        (F.col("price") * Fees.KUCOIN_BTC_TRADE_FEE - Fees.KUCOIN_BTC_WITHDRAW_FEE) * Fees.BITHUMB_BTC_TRADE_FEE
    ) - F.col("price")
    kucoin_bithumb_eth = (
        (F.col("price") * Fees.KUCOIN_ETH_TRADE_FEE - Fees.KUCOIN_ETH_WITHDRAW_FEE) * Fees.BITHUMB_ETH_TRADE_FEE
    ) - F.col("price")

    # 각 거래소 쌍의 매매 차익 값을 DataFrame에 추가
    profit_df = (
        df.withColumn("profit_binance_upbit_btc", binance_upbit_btc)
        .withColumn("profit_binance_upbit_eth", binance_upbit_eth)
        .withColumn("profit_binance_bithumb_btc", binance_bithumb_btc)
        .withColumn("profit_binance_bithumb_eth", binance_bithumb_eth)
        .withColumn("profit_kucoin_upbit_btc", kucoin_upbit_btc)
        .withColumn("profit_kucoin_upbit_eth", kucoin_upbit_eth)
        .withColumn("profit_kucoin_bithumb_btc", kucoin_bithumb_btc)
        .withColumn("profit_kucoin_bithumb_eth", kucoin_bithumb_eth)
        .withColumn("timestamp", F.current_timestamp())
    )

    # 각 거래소 쌍별로 symbol, timestamp, 위에서 계산된 profit_value를 포함한 DataFrame 생성
    arbitrage_values_df = profit_df.select(
        "symbol",  # symbol 열 유지
        "timestamp",  # timestamp 열 유지
        F.explode(
            F.array(
                F.struct(
                    F.lit("Binance-Upbit-BTC").alias("exchange_pair"),
                    F.col("profit_binance_upbit_btc").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Binance-Upbit-ETH").alias("exchange_pair"),
                    F.col("profit_binance_upbit_eth").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Binance-Bithumb-BTC").alias("exchange_pair"),
                    F.col("profit_binance_bithumb_btc").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Binance-Bithumb-ETH").alias("exchange_pair"),
                    F.col("profit_binance_bithumb_eth").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Kucoin-Upbit-BTC").alias("exchange_pair"),
                    F.col("profit_kucoin_upbit_btc").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Kucoin-Upbit-ETH").alias("exchange_pair"),
                    F.col("profit_kucoin_upbit_eth").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Kucoin-Bithumb-BTC").alias("exchange_pair"),
                    F.col("profit_kucoin_bithumb_btc").alias("profit_value"),
                ),
                F.struct(
                    F.lit("Kucoin-Bithumb-ETH").alias("exchange_pair"),
                    F.col("profit_kucoin_bithumb_eth").alias("profit_value"),
                ),
            )
        ).alias("profit_struct"),
    ).select("symbol", "profit_struct.exchange_pair", "profit_struct.profit_value", "timestamp")

    return arbitrage_values_df
