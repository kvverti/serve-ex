//! Contains data structures that support this application. Note that because serde only handles serialization (not validation), the <*>::is_acceptable methods
//! are present to determine whether the data structure makes semantic (rather than syntactic) sense.

use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use time::{Date, Time};

/// Contains serialization/deserialization helpers for data types.
mod serialization;

/// A price on a receipt containing dollars and cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    pub dollars: u64,
    pub cents: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// The short product description for the item.
    pub short_description: String,
    /// The total price paid for this item.
    pub price: Price,
}

impl Item {
    /// Determines whether this item is acceptable. An item is acceptable if the short description contains only words.
    pub fn is_acceptable(&self) -> bool {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX
            .get_or_init(|| Regex::new(r"^[\w\s-]+$").expect("description regex should be valid"));

        regex.is_match(&self.short_description)
    }
}

/// A receipt.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub retailer: String,
    #[serde(with = "serialization::date")]
    pub purchase_date: Date,
    #[serde(with = "serialization::time")]
    pub purchase_time: Time,
    pub items: Vec<Item>,
    pub total: Price,
}

impl Receipt {
    /// Determines whether this receipt is acceptable. Receipts must fulfill these requirements to be acceptable:
    /// - the receipt must have at least one item
    /// - all items must be acceptable
    /// - the retailer name must contain only words
    pub fn is_acceptable(&self) -> bool {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX
            .get_or_init(|| Regex::new(r"^[\w\s&-]+$").expect("retailer regex should be valid"));

        regex.is_match(&self.retailer)
            && !self.items.is_empty()
            && self.items.iter().all(Item::is_acceptable)
    }
}
