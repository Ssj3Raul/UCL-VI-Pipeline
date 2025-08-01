use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rumqttc::{MqttOptions, AsyncClient, Event, Packet};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mqtt_host = env::var("MQTT_HOST").unwrap_or("test.mosquitto.org".to_string());
    let mqtt_port = env::var("MQTT_PORT").unwrap_or("1883".to_string()).parse::<u16>().unwrap();
    let mqtt_topic = env::var("MQTT_TOPIC").unwrap_or("test/vi-sample".to_string());

    let kafka_broker = env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or("test-topic".to_string());

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_broker)
        .create()
        .expect("Kafka producer creation error");

    // ---- Setup MQTT options
    let mut mqttoptions = MqttOptions::new("collector", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));

    // ---- Create an Async MQTT client and get its eventloop
    let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // ---- Subscribe to topic
    mqtt_client.subscribe(mqtt_topic.clone(), rumqttc::QoS::AtMostOnce).await.unwrap();

    println!("Subscribed to MQTT topic: {}", mqtt_topic);

    // ---- Main async event loop
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(publish))) => {
                println!("MQTT RAW BYTES: {:?}", publish.payload);

                // For debugging: print as string if valid UTF-8
                if let Ok(msg) = std::str::from_utf8(&publish.payload) {
                    println!("Received MQTT message: {:?}", msg);
                } else {
                    println!("Received MQTT message: <invalid UTF-8>");
                }

                // Forward raw bytes to Kafka!
                let record = FutureRecord::to(&kafka_topic)
                    .payload(publish.payload.as_ref())
                    .key(&[]);

                println!("FORWARDING TO KAFKA (RAW): {:?}", publish.payload);

                match producer.send(record, Duration::from_secs(0)).await {
                    Ok(delivery) => println!("Delivered to Kafka: {:?}", delivery),
                    Err((e, _)) => eprintln!("Failed to deliver to Kafka: {:?}", e),
                }
            }
            Ok(_) => {} // Ignore other event types
            Err(e) => {
                eprintln!("MQTT event loop error: {:?}", e);
                break;
            }
        }
    }
}
