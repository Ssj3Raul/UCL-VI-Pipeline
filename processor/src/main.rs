use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sqlx::{PgPool, query};
use std::env;
use uuid::Uuid;
use serde::Deserialize;
use time::OffsetDateTime;


#[derive(Deserialize, Debug)]
struct SensorReading {
    sensorId: String,
    timestamp: String, // You can parse this into chrono::DateTime<Utc> if you want
    value: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let kafka_broker = env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or("test-topic".to_string());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = PgPool::connect(&db_url).await?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", &kafka_broker)
        .create()
        .expect("Consumer creation error");

    consumer.subscribe(&[&kafka_topic]).expect("Failed to subscribe to topic");

    println!("Processor waiting for messages on topic: {}", kafka_topic);

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let payload = msg.payload_view::<str>().unwrap_or(Ok("")).unwrap_or("");
                println!("Received: {}", payload);

                if let Ok(reading) = serde_json::from_str::<SensorReading>(payload) {
                    // Parse UUID and (optionally) timestamp
                    let sensor_id = Uuid::parse_str(&reading.sensorId)?;

                    // NEW: Parse timestamp from the string in JSON
                    let parsed_time = OffsetDateTime::parse(&reading.timestamp, &time::format_description::well_known::Rfc3339)?;

                    let value = reading.value;
                    // Parse timestamp string into chrono::DateTime<Utc> if needed

                    let res = query!(
                        "INSERT INTO reading (id, sensorId, timestamp, rawValue, value, createdAt)
                         VALUES ($1, $2, $3, $4, $5, NOW())",
                        Uuid::new_v4(),
                        sensor_id,
                        parsed_time,
                        value,
                        value
                    )
                    .execute(&db)
                    .await;

                    match res {
                        Ok(_) => println!("Inserted into DB!"),
                        Err(e) => eprintln!("DB insert error: {:?}", e),
                    }
                } else {
                    eprintln!("Failed to parse payload as JSON: {:?}", payload);
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
