use std::sync::OnceLock;

use regex::Regex;
use serde::{de::Visitor, Deserialize, Serialize};

use super::Price;

pub mod date;
pub mod time;

fn price_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^(\d+)\.(\d{2})$").expect("price regex should be valid"))
}

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = format!("{:01}.{:02}", self.dollars, self.cents);
        serializer.serialize_str(&str_value)
    }
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PriceVisitor;
        impl<'de> Visitor<'de> for PriceVisitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a numeric string with two digits after the decimal"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let Some(captures) = price_regex().captures(v) else {
                    return Err(E::custom("invalid price string"));
                };
                let (_, [dollars, cents]) = captures.extract();
                let dollars: u64 = dollars
                    .parse()
                    .map_err(|_| E::custom("dollars amount is invalid"))?;
                let cents: u8 = cents.parse().expect("cents should have been validated earlier");
                Ok(Price { dollars, cents })
            }
        }

        deserializer.deserialize_str(PriceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use ::time::{Date, Time};
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn price() {
        let prices = (
            Price {
                dollars: 0,
                cents: 50,
            },
            Price {
                dollars: 3,
                cents: 0,
            },
            Price {
                dollars: 10,
                cents: 7,
            },
            Price {
                dollars: 999,
                cents: 99,
            },
        );
        assert_tokens(
            &prices,
            &[
                Token::Tuple { len: 4 },
                Token::Str("0.50"),
                Token::Str("3.00"),
                Token::Str("10.07"),
                Token::Str("999.99"),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn date() {
        /// Wrapper used so serde knows to use our custom serialization.
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        struct MyDate(#[serde(with = "super::date")] Date);

        use ::time::macros::date as d;
        let dates = (
            MyDate(d!(2001 - 01 - 01)),
            MyDate(d!(2024 - 10 - 17)),
            MyDate(d!(2000 - 02 - 29)),
        );
        assert_tokens(
            &dates,
            &[
                Token::Tuple { len: 3 },
                Token::NewtypeStruct { name: "MyDate" },
                Token::Str("2001-01-01"),
                Token::NewtypeStruct { name: "MyDate" },
                Token::Str("2024-10-17"),
                Token::NewtypeStruct { name: "MyDate" },
                Token::Str("2000-02-29"),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn time() {
        /// Wrapper used so serde knows to use our custom serialization.
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        struct MyTime(#[serde(with = "super::time")] Time);

        use ::time::macros::time as t;
        let dates = (
            MyTime(t!(00:00)),
            MyTime(t!(09:30)),
            MyTime(t!(12:45)),
            MyTime(t!(17:15)),
        );
        assert_tokens(
            &dates,
            &[
                Token::Tuple { len: 4 },
                Token::NewtypeStruct { name: "MyTime" },
                Token::Str("00:00"),
                Token::NewtypeStruct { name: "MyTime" },
                Token::Str("09:30"),
                Token::NewtypeStruct { name: "MyTime" },
                Token::Str("12:45"),
                Token::NewtypeStruct { name: "MyTime" },
                Token::Str("17:15"),
                Token::TupleEnd,
            ],
        );
    }
}
