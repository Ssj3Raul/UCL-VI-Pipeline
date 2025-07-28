use rdkafka::config::ClientConfig;
use rdkafka::producer::Producer;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Load .env variables (if present)
    dotenv::dotenv().ok();

    // Read Kafka connection info from env
    let kafka_broker = env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or_else(|_| "test-topic".to_string());

    println!("Connecting to Kafka at: {}", kafka_broker);
    println!("Publishing to topic: {}", kafka_topic);

    // Create the producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_broker)
        .create()
        .expect("Producer creation error");

    // Payload to send
    let payload = r#"{"sensor": "Room101", "value": 22.5}"#;
    let record = FutureRecord::<(), _>::to(&kafka_topic).payload(payload);

    // Send record and handle result
    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Delivered: {:?}", delivery),
        Err((e, _)) => eprintln!("Failed to deliver: {:?}", e),
    }

    // Flush to make sure all messages are sent (best practice if you later send >1 message)
    let _ = producer.flush(Duration::from_secs(1));
}
