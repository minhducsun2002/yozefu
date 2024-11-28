use apache_avro::types::Value;
use serde_json::{Map, Number};

/// Converts an Avro value to a JSON value.
pub(crate) fn avro_to_json(value: Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(b),
        Value::Int(i) => serde_json::Value::Number(Number::from(i)),
        Value::Long(l) => serde_json::Value::Number(Number::from(l)),
        Value::Float(f) => serde_json::Value::Number(Number::from_f64(f.into()).unwrap()),
        Value::Double(f) => serde_json::Value::Number(Number::from_f64(f).unwrap()),
        Value::Bytes(vec) => serde_json::Value::Array(
            vec.iter()
                .map(|b| serde_json::Value::Number(Number::from(*b)))
                .collect(),
        ),
        Value::String(s) => serde_json::Value::String(s),
        Value::Fixed(_, vec) => serde_json::Value::Array(
            vec.iter()
                .map(|b| serde_json::Value::Number(Number::from(*b)))
                .collect(),
        ),
        Value::Enum(_, s) => serde_json::Value::String(s),
        Value::Union(_, value) => avro_to_json(*value),
        Value::Array(vec) => {
            serde_json::Value::Array(vec.iter().map(|v| avro_to_json(v.clone())).collect())
        }
        Value::Map(hash_map) => serde_json::Value::Object(
            hash_map
                .into_iter()
                .map(|(k, v)| (k, avro_to_json(v)))
                .collect(),
        ),
        Value::Record(vec) => {
            serde_json::Value::Object(vec.into_iter().map(|(k, v)| (k, avro_to_json(v))).collect())
        }
        Value::Date(date) => serde_json::Value::Number(Number::from(date)),
        Value::TimeMillis(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::TimeMicros(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::TimestampMillis(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::TimestampMicros(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::TimestampNanos(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::LocalTimestampMillis(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::LocalTimestampMicros(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::LocalTimestampNanos(ts) => serde_json::Value::Number(Number::from(ts)),
        Value::Uuid(uuid) => serde_json::Value::String(uuid.to_string()),
        Value::Duration(duration) => {
            let mut map = Map::with_capacity(3);
            let i: u32 = duration.months().into();
            map.insert("months".to_string(), serde_json::Value::Number(i.into()));
            let i: u32 = duration.millis().into();
            map.insert("millis".to_string(), serde_json::Value::Number(i.into()));
            let i: u32 = duration.days().into();
            map.insert("days".to_string(), serde_json::Value::Number(i.into()));
            serde_json::Value::Object(map)
        }
        Value::Decimal(_decimal) => serde_json::Value::String(
            "Yozefu error: I don't know how to encode a decimal to json. It fails silently"
                .to_string(),
        ),
        Value::BigDecimal(big_decimal) => serde_json::Value::String(big_decimal.to_string()),
    }
}
