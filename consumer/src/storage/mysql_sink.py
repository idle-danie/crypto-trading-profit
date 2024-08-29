from pyspark.sql import DataFrame
from config.config import Config


def write_to_mysql(df: DataFrame, table_name: str):
    """
    MySQL에 데이터를 적재하는 함수.
    :param df: MySQL에 쓰기 작업이 진행될 Spark DataFrame
    :param table_name: 적재할 MySQL 테이블 명
    """
    return (
        df.writeStream.foreachBatch(
            lambda batch_df, _: (
                batch_df.show(5),  # 배치 데이터 확인
                batch_df.write.format("jdbc")
                .option("url", f"jdbc:mysql://{Config.MYSQL_HOST}:{Config.MYSQL_PORT}/{Config.MYSQL_DB}")
                .option("driver", "com.mysql.cj.jdbc.Driver")
                .option("dbtable", table_name)
                .option("user", Config.MYSQL_USER)
                .option("password", Config.MYSQL_PASSWORD)
                .mode("append")
                .save(),
            )
        )
        .outputMode("append")
        .start()
    )
