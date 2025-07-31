mod normalisation;
use crate::normalisation::{NormalisedReading, normalise_reading};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sqlx::{PgPool, query};
use std::env;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
                

                // Try to parse *any* JSON (messy or clean)
                if let Ok(raw_json) = serde_json::from_str::<serde_json::Value>(payload) {
                    match normalisation::normalise_reading(&raw_json) {
                        Ok(reading) => {
                            println!("Normalised reading: {:?}", reading);

                            // Lookup sensorId in the sensor table
                            let sensor_row = sqlx::query!(
                                "SELECT id FROM sensor WHERE identifier = $1 AND measuring = $2",
                                &reading.identifier,
                                &reading.measuring
                            )
                            .fetch_optional(&db)
                            .await?;

                            if let Some(record) = sensor_row {
                                let sensor_id = record.id;
                                let val = reading.value.unwrap_or(0.0);

                                let res = query!(
                                    "INSERT INTO reading (id, sensorId, timestamp, rawValue, value, createdAt)
                                     VALUES ($1, $2, $3, $4, $5, NOW())",
                                    Uuid::new_v4(),
                                    sensor_id,
                                    reading.timestamp,
                                    val, // rawValue
                                    val  // value
                                )
                                .execute(&db)
                                .await;

                                match res {
                                    Ok(_) => println!("Inserted into DB!"),
                                    Err(e) => eprintln!("DB insert error: {:?}", e),
                                }
                            } else {
                                eprintln!(
                                    "Sensor not found in DB for identifier: {}, measuring: {}",
                                    &reading.identifier, &reading.measuring
                                );
                                // Optionally: alert, skip, or handle dead-letter queue here
                            }
                        }
                        Err(e) => eprintln!("Normalisation error: {:?}", e),
                    }
                } else {
                    eprintln!("Failed to parse payload as JSON: {:?}", payload);
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
