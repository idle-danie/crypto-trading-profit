use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::env;

pub fn create_producer() -> FutureProducer {
    let kafka_bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")
        .expect("KAFKA_BOOTSTRAP_SERVERS 환경 변수를 설정하세요.");

    ClientConfig::new()
        .set("bootstrap.servers", &kafka_bootstrap_servers)
        .create()
        .expect("Producer creation error")
}

pub async fn send_to_kafka(producer: &FutureProducer, topic: &str, symbol: &str, payload: &[u8]) {
    producer.send(
        FutureRecord::to(topic)
            .key(symbol)
            .payload(payload),
        Timeout::Never,
    ).await.expect("Failed to send message to Kafka");
}
