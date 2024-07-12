//! Custom serialization for Date is required because the application does not use a well-known format.

use serde::de::Visitor;
use time::{Date, macros::format_description};

/// Serializes a Date to a yyyy-MM-dd string.
pub fn serialize<S: serde::Serializer>(v: &Date, serializer: S) -> Result<S::Ok, S::Error> {
    let description = format_description!("[year]-[month]-[day]");
    let formatted_date = v.format(description).expect("date should be able to be formatted");
    serializer.serialize_str(&formatted_date)
}

/// Deserializes a date from a yyyy-MM-dd string.
pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
    struct DateVisitor;
    impl<'de> Visitor<'de> for DateVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a date in yyyy-mm-dd format")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            let description = format_description!("[year]-[month]-[day]");
            Date::parse(v, description).map_err(|_| E::custom("invalid date"))
        }
    }

    deserializer.deserialize_str(DateVisitor)
}