# Robust IoT Pipeline

A modular Rust-based pipeline for ingesting, processing, and storing IoT sensor data—matching VI’s real system architecture.

---

## 🚀 Features

- **Collector**: Rust binary that sends IoT (test/demo) messages to Kafka.
- **Processor**: Rust binary that consumes messages from Kafka and writes them into PostgreSQL.
- **PostgreSQL**: Stores sensor readings, graph definitions, and sensor metadata.
- **Kafka**: Reliable buffer and data streaming backbone.
- **Docker Compose**: For quick local development and testing.

---

## 🛠️ Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Rust](https://rustup.rs/)
- (Recommended) [Git](https://git-scm.com/) or you can unzip the provided folder

---

## ⚡ Getting Started

### 1. Clone or Unzip

```bash
git clone <your-repo-url>
cd robust-iot-pipeline

### 2. Start PostgreSQL & Kafka

docker-compose up -d

###3. Create the Database Schema and setup fake sensor

docker cp schema.sql iot-postgres:/schema.sql

docker exec -it iot-postgres psql -U vi_user vi_graphs

# Inside psql prompt:
\i /schema.sql

SELECT * FROM sensor
INSERT INTO sensor (id, identifier, measuring, unit)
VALUES ('11111111-1111-1111-1111-111111111111', 'Room101', 'temperature', 'C');

# Then exit psql:
\q

###4. build rust project

cargo build

###6. Run the Processor

cargo run --bin processor

###7. Run the Collector (in another terminal)

cargo run --bin collector

###8. 8. Check the Data in Postgres

docker exec -it iot-postgres psql -U vi_user vi_graphs
SELECT * FROM reading;

###Sometimes the cargo enviornment gets buggy, do the floowing to clean it:

cargo clean
