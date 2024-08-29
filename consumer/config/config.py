import os
from dotenv import load_dotenv

load_dotenv()


class Config:
    KAFKA_BROKER_URL = os.getenv("KAFKA_BROKER_URL")
    KAFKA_BINANCE_TOPIC = os.getenv("KAFKA_BINANCE_TOPIC")
    KAFKA_KUCOIN_TOPIC = os.getenv("KAFKA_KUCOIN_TOPIC")
    KAFKA_UPBIT_TOPIC = os.getenv("KAFKA_UPBIT_TOPIC")
    KAFKA_BITHUMB_TOPIC = os.getenv("KAFKA_BITHUMB_TOPIC")

    MYSQL_HOST = os.getenv("MYSQL_HOST")
    MYSQL_PORT = os.getenv("MYSQL_PORT")
    MYSQL_DB = os.getenv("MYSQL_DB")
    MYSQL_USER = os.getenv("MYSQL_USER")
    MYSQL_PASSWORD = os.getenv("MYSQL_PASSWORD")

    BATCH_DURATION = int(os.getenv("BATCH_DURATION", 5))
