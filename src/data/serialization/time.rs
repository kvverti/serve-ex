//! Custom serialization for Time is required because the application does not use a well-known format.

use serde::de::Visitor;
use time::{Time, macros::format_description};

/// Serializes a Time to a HH:mm string.
pub fn serialize<S: serde::Serializer>(v: &Time, serializer: S) -> Result<S::Ok, S::Error> {
    let description = format_description!("[hour repr:24]:[minute]");
    let formatted_time = v.format(description).expect("time should be able to be formatted");
    serializer.serialize_str(&formatted_time)
}

/// Deserializes a Time from a HH:mm string.
pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Time, D::Error> {
    struct TimeVisitor;
    impl<'de> Visitor<'de> for TimeVisitor {
        type Value = Time;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a time in hh:mm format")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            let description = format_description!("[hour repr:24]:[minute]");
            Time::parse(v, description).map_err(|_| E::custom("invalid time"))
        }
    }

    deserializer.deserialize_str(TimeVisitor)
}