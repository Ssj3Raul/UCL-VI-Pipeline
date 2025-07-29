use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use std::env;
use std::time::Duration;
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let kafka_broker = env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or("test-topic".to_string());
    let sensor_id = env::var("SENSOR_ID").unwrap_or("11111111-1111-1111-1111-111111111111".to_string());
    let value = env::var("SENSOR_VALUE").unwrap_or("23.7".to_string()).parse::<f64>().unwrap_or(23.7);

    // Current time in RFC3339 format
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Build the payload as a JSON object
    let payload = json!({
        "sensorId": sensor_id,
        "timestamp": timestamp,
        "value": value
    }).to_string();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_broker)
        .create()
        .expect("Producer creation error");

    let record = FutureRecord::to(&kafka_topic)
        .payload(&payload)
        .key(&());

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Delivered: {:?}", delivery),
        Err((e, _)) => eprintln!("Failed to deliver: {:?}", e),
    }

    let _ = producer.flush(Duration::from_secs(1));
}
