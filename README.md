# Robust IoT Pipeline

A modular Rust-based data pipeline for ingesting, processing, and storing IoT sensor data—designed for containerized deployment and eventual Kubernetes migration.

## 🏗️ Architecture

```
MQTT Sources → Collector → Kafka → Processor → PostgreSQL
```

### Components

- **Collector**: Rust binary that ingests IoT sensor data from MQTT and forwards to Kafka
- **Processor**: Rust binary that consumes messages from Kafka, normalizes data, and stores in PostgreSQL
- **PostgreSQL**: Stores sensor readings, graph definitions, and sensor metadata
- **Kafka**: Reliable message streaming backbone with Zookeeper
- **Docker Compose**: Complete local development environment

## 🚀 Features

- **Containerized**: Each component runs in its own Docker container
- **Scalable**: Designed for horizontal scaling in Kubernetes
- **Fault-tolerant**: Retry logic and error handling built-in
- **Production-ready**: Proper logging, health checks, and monitoring
- **Developer-friendly**: Hot reloading and easy local development

## 📋 Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Rust](https://rustup.rs/) (for local development)
- [Git](https://git-scm.com/)

## 🛠️ First-Time Installation

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd robust-iot-pipeline
```

### 2. Start Infrastructure

```bash
# Start PostgreSQL, Kafka, and Zookeeper
docker-compose up -d postgres zookeeper kafka
```

### 3. Initialize Database

```bash
# Create database schema and sample sensor
docker cp schema.sql iot-postgres:/schema.sql

docker exec -it iot-postgres psql -U vi_user vi_graphs

# Inside psql prompt:
\i /schema.sql

SELECT * FROM sensor
INSERT INTO sensor (id, identifier, measuring, unit)
VALUES ('11111111-1111-1111-1111-111111111111', 'Room101', 'temperature', 'C');

# Then exit psql:
\q
```

### 4. Build Applications

```bash
# Build collector and processor containers
docker-compose build collector processor
```

## 🔄 Development Workflow

### Option A: Full Docker Development (Recommended)

```bash
# 1. Start infrastructure (if not running)
docker-compose up -d postgres zookeeper kafka

# 2. Start processor FIRST (important for Kafka consumer groups)
docker-compose up -d processor

# 3. Start collector SECOND
docker-compose up -d collector

# 4. Monitor logs
docker-compose logs -f processor collector

# 5. Send test messages
mosquitto_pub -h test.mosquitto.org -t test/vi-sample -f payload.json
```

### Option B: Hybrid Development (Faster iteration)

```bash
# 1. Start infrastructure in Docker
docker-compose up -d postgres zookeeper kafka

# 2. Run processor locally (faster rebuilds)
$env:KAFKA_BROKER="localhost:9092"; $env:DATABASE_URL="postgres://vi_user:vi_pass@localhost:5432/vi_graphs"; cargo run --bin processor

# 3. Run collector locally (in another terminal)
$env:KAFKA_BROKER="localhost:9092"; cargo run --bin collector

# 4. Send test messages
mosquitto_pub -h test.mosquitto.org -t test/vi-sample -f payload.json
```

## 🧪 Testing

### Send Test Messages

```bash
# Single message
mosquitto_pub -h test.mosquitto.org -t test/vi-sample -f payload.json

# Multiple messages
for ($i=1; $i -le 5; $i++) { 
    mosquitto_pub -h test.mosquitto.org -t test/vi-sample -f payload.json; 
    Start-Sleep -Seconds 2 
}

# Custom message
mosquitto_pub -h test.mosquitto.org -t test/vi-sample -m '{"identifier":"Room102","measuring":"humidity","value":65,"value_type":"Number","unit":"%","timestamp":"2025-08-06T20:50:00Z"}'
```

### Monitor the Pipeline

```bash
# View all logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f processor
docker-compose logs -f collector

# Check database
docker exec -it iot-postgres psql -U vi_user vi_graphs -c "SELECT * FROM reading;"
```

## 📊 Data Flow

1. **MQTT Message**: Sensor data published to `test/vi-sample`
2. **Collector**: Receives MQTT message and forwards to Kafka
3. **Kafka**: Buffers and streams the message
4. **Processor**: Consumes from Kafka, normalizes data, stores in PostgreSQL
5. **PostgreSQL**: Persists the sensor reading

### Expected Logs

**Collector:**
```
Subscribed to MQTT topic: test/vi-sample
Received MQTT message: {...}
Delivered to Kafka: (0, 1)
```

**Processor:**
```
Processor waiting for messages on topic: test-topic
Received: {...}
Normalised reading: NormalisedReading {...}
Inserted into DB!
```

## 🛠️ Common Commands

```bash
# Start everything
docker-compose up -d

# Stop everything
docker-compose down

# Rebuild after code changes
docker-compose build processor
docker-compose up processor

# Check running containers
docker-compose ps

# View logs
docker-compose logs -f

# Restart a service
docker-compose restart processor
```

## 🔧 Troubleshooting

### Processor Not Receiving Messages

1. **Check startup order**: Processor must start before collector
2. **Verify Kafka connection**: Check processor logs for connection errors
3. **Check consumer group**: Ensure processor is properly subscribed

### Database Connection Issues

1. **Verify PostgreSQL is running**: `docker-compose ps postgres`
2. **Check connection string**: Ensure `DATABASE_URL` is correct
3. **Wait for database**: Add retry logic if needed

### MQTT Connection Issues

1. **Check network**: Ensure collector can reach `test.mosquitto.org`
2. **Verify topic**: Ensure topic matches between publisher and subscriber
3. **Check message format**: Ensure JSON is valid

## 📁 Project Structure

```
robust-iot-pipeline/
├── collector/           # MQTT to Kafka service
│   ├── src/main.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── processor/           # Kafka to PostgreSQL service
│   ├── src/main.rs
│   ├── src/normalisation.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── common/             # Shared code
│   ├── src/lib.rs
│   └── Cargo.toml
├── docker-compose.yml  # Local development environment
├── schema.sql         # Database schema
├── payload.json       # Sample test data
└── README.md
```

## 🚀 Production Deployment

This pipeline is designed for Kubernetes deployment:

- **Containerized**: Each component runs in its own pod
- **Scalable**: Can run multiple instances of collector/processor
- **Configurable**: Environment-based configuration
- **Observable**: Structured logging and metrics ready

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with the development workflow
5. Submit a pull request

## 📄 License

[Your License Here]

---

**Happy IoT Data Processing! 🚀**
