use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserializer, Serializer, Deserialize};

/// Special function to deserialize Option<DateTime<Utc>>
pub fn deserialize_option<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where D: Deserializer<'de>
{
    let dt: Option<i64> = Option::deserialize(d)?;
    if let Some(dt) = dt {
        return Ok(Some(
            Utc.timestamp_opt(dt / 1000,
                              ((dt % 1000) * 1_000_000) as u32).unwrap()
        ));
    }
    Ok(None)
}

/// Special function to serialize Option<DateTime<Utc>>
pub fn serialize_option<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    return if dt.is_some() {
        serializer.serialize_i64(dt.unwrap().timestamp())
    } else {
        serializer.serialize_none()
    }
}

