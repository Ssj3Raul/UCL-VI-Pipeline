use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sqlx::{PgPool, query};
use std::env;
use uuid::Uuid;

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

                // For now, use dummy values (parse real payload later!)
                let sensor_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(); // TODO: parse actual sensor_id from payload
                let value: f64 = 22.5; // TODO: parse value from payload

                let res = query!(
                    "INSERT INTO reading (id, sensorId, timestamp, rawValue, value, createdAt)
                     VALUES ($1, $2, NOW(), $3, $4, NOW())",
                    Uuid::new_v4(),
                    sensor_id,
                    value,
                    value
                )
                .execute(&db)
                .await;

                match res {
                    Ok(_) => println!("Inserted into DB!"),
                    Err(e) => eprintln!("DB insert error: {:?}", e),
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
