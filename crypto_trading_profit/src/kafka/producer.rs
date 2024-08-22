use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
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