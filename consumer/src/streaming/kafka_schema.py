from pyspark.sql.types import StructType, StructField, StringType, FloatType


def kafka_message_schema():
    return StructType(
        [
            StructField("symbol", StringType()),
            StructField("exchange_name", StringType()),
            StructField("opening_price", FloatType()),
            StructField("high_price", FloatType()),
            StructField("low_price", FloatType()),
            StructField("last_price", FloatType()),
            StructField("volume", FloatType()),
            StructField("price_change", FloatType()),
            StructField("price_change_percent", FloatType()),
        ]
    )
