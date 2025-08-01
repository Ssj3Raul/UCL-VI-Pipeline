// processor/src/normalisation.rs

use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub enum IotFieldType {
    Number,
    Boolean,
    String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IotUnit {
    DegreesCelsius,
    Fahrenheit,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NormalisedReading {
    pub identifier: String,
    pub measuring: String,
    pub value: Option<f64>,
    pub raw_value: String,
    pub value_type: IotFieldType,
    pub unit: IotUnit,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<Map<String, Value>>,
}

pub fn normalise_reading(raw: &Value) -> Result<NormalisedReading, String> {
    // 1. Extract required fields (with fallbacks)
    let identifier = raw.get("identifier")
        .and_then(Value::as_str)
        .unwrap_or("unknown_sensor")
        .to_string();

    let measuring = raw.get("measuring")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();

    // 2. Value type & unit
    let value_type = match raw.get("value_type").and_then(Value::as_str) {
        Some("Number") | Some("NUMBER") => IotFieldType::Number,
        Some("Boolean") | Some("BOOLEAN") => IotFieldType::Boolean,
        Some("String") | Some("STRING") => IotFieldType::String,
        _ => IotFieldType::Number,
    };

    let unit = match raw.get("unit").and_then(Value::as_str) {
        Some("DegreesCelsius") | Some("DEGREES_CELCIUS") => IotUnit::DegreesCelsius,
        Some("Fahrenheit") | Some("FAHRENHEIT") => IotUnit::Fahrenheit,
        _ => IotUnit::Unknown,
    };

    // 3. Raw value as string
    let raw_value = raw.get("value")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "".to_string());

    // 4. Parse value to f64 if possible
    let value = match raw.get("value") {
        Some(v) => match v {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse::<f64>().ok(),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None
        },
        None => None
    };

    // 5. Parse timestamp (RFC3339 preferred, fallback to now)
    let timestamp_str = raw.get("timestamp")
        .and_then(Value::as_str)
        .unwrap_or("");
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    // 6. Metadata (optional)
    let metadata = raw.get("metadata").and_then(Value::as_object).cloned();

    Ok(NormalisedReading {
        identifier,
        measuring,
        value,
        raw_value,
        value_type,
        unit,
        timestamp,
        metadata,
    })
}
