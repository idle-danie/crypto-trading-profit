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

    query_crypto_prices = processed_stream.writeStream.foreachBatch(lambda df, _: df.show()).start()

    # MySQL에 crypto_prices 테이블에 데이터 적재
    query_crypto_prices_to_mysql = write_to_mysql(processed_stream, "crypto_prices")

    # 매매 차익 계산 (워터마크 적용된 데이터로 처리)
    arbitrage_avg_df = calculate_arbitrage_average(processed_stream)
    arbitrage_values_df = calculate_arbitrage_values(processed_stream)

    # 각 단계에서 데이터 출력
    query_arbitrage_avg_show = arbitrage_avg_df.writeStream.foreachBatch(lambda df, _: df.show()).start()
    query_arbitrage_values_show = arbitrage_values_df.writeStream.foreachBatch(lambda df, _: df.show()).start()

    # MySQL에 데이터 적재
    query_arbitrage_avg = write_to_mysql(arbitrage_avg_df, "arbitrage_average")
    query_arbitrage_values = write_to_mysql(arbitrage_values_df, "arbitrage_values")

    # 각 스트리밍 쿼리의 종료를 기다림
    query_crypto_prices.awaitTermination()
    query_crypto_prices_to_mysql.awaitTermination()
    query_arbitrage_avg.awaitTermination()
    query_arbitrage_values.awaitTermination()
    query_arbitrage_avg_show.awaitTermination()
    query_arbitrage_values_show.awaitTermination()
