from pyspark.sql import functions as F
from streaming.kafka_schema import kafka_message_schema
from processing.data_processing import calculate_arbitrage_average, calculate_arbitrage_values, process_kafka_stream
from storage.mysql_sink import write_to_mysql
from config.config import Config


def start_streaming(spark):
    kafka_params = {
        "kafka.bootstrap.servers": Config.KAFKA_BROKER_URL,
        "subscribe": f"{Config.KAFKA_BINANCE_TOPIC},{Config.KAFKA_KUCOIN_TOPIC},{Config.KAFKA_UPBIT_TOPIC},{Config.KAFKA_BITHUMB_TOPIC}",
    }

    kafka_stream = spark.readStream.format("kafka").options(**kafka_params).load()

    parsed_stream = (
        kafka_stream.selectExpr("CAST(value AS STRING) as message")
        .select(F.from_json("message", kafka_message_schema()).alias("data"))
        .select("data.*")
    )
    processed_stream = process_kafka_stream(parsed_stream)

    # 중복 제거 후 MySQL에 적재
    deduplicated_stream = processed_stream.dropDuplicates(["symbol", "timestamp", "exchange_name"])

    query_crypto_prices_to_mysql = (
        deduplicated_stream.writeStream.outputMode("append")
        .foreachBatch(lambda df, epochId: write_to_mysql(df, "crypto_prices"))
        .start()
    )

    # 매매 차익 average와 values 산출 (워터마크 적용된 데이터로 처리)
    arbitrage_avg_df = calculate_arbitrage_average(deduplicated_stream)
    arbitrage_values_df = calculate_arbitrage_values(deduplicated_stream)

    # 중복 제거 후 MySQL에 적재
    query_arbitrage_avg = (
        arbitrage_avg_df.writeStream.outputMode("append")
        .foreachBatch(lambda df, epochId: write_to_mysql(df, "arbitrage_average"))
        .start()
    )

    query_arbitrage_values = (
        arbitrage_values_df.writeStream.outputMode("append")
        .foreachBatch(
            lambda df, epochId: write_to_mysql(
                df.dropDuplicates(["symbol", "exchange_pair", "timestamp"]), "arbitrage_values"
            )
        )
        .start()
    )

    query_crypto_prices_to_mysql.awaitTermination()
    query_arbitrage_avg.awaitTermination()
    query_arbitrage_values.awaitTermination()
