from pyspark.sql import SparkSession
from streaming.streaming_job import start_streaming


def create_spark_session():
    spark = SparkSession.builder.appName("Crypto Arbitrage Streaming").getOrCreate()
    spark.sparkContext.setLogLevel("ERROR")  # 로그 레벨: ERROR
    return spark


if __name__ == "__main__":
    # Spark 세션 생성
    spark = create_spark_session()

    # 스트리밍 작업 시작
    start_streaming(spark)
