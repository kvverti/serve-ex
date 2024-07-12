use std::num::Saturating;

use actix_web::{get, web, HttpResponse};
use serde::{Serialize, Deserialize};
use time::macros::time;
use uuid::Uuid;

use crate::{data::Receipt, AppState};

/// Calculates points for the receipt, as follows:
/// - 1 pt for each letter or numeral in the retailer name
/// - 50 pt if the total has .00 cents
/// - 25 pt if the total is a multiple of .25 cents
/// - 5 pt for every two items (e.g. 10 items = 5 pt, 3 items = 1 pt)
/// - if the (trimmed) length of an item description is a multiple of 3, add points according to
///   ceil(price * 0.2)
/// - 6 pt if the day of the purchase date is odd
/// - 10 pt if the time of the purchase is between 14:00 and 16:00 (exclusive)
fn calculate_points(receipt: &Receipt) -> u64 {
    // overflow is unlikely, but if it happens we just saturate
    let mut total_pts = Saturating(0u64);

    let letter_digit_count: u64 = receipt
        .retailer
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .count()
        .try_into()
        .expect("length should fit in 64 bits");
    total_pts += letter_digit_count;

    if receipt.total.cents == 0 {
        total_pts += 50;
    }

    if receipt.total.cents % 25 == 0 {
        total_pts += 25;
    }

    let item_pts =
        5 * u64::try_from(receipt.items.len() / 2).expect("length should fit in 64 bits");
    total_pts += item_pts;

    for item in receipt.items.iter() {
        if item.short_description.trim().len() % 3 == 0 {
            let float_price = item.price.dollars as f64 + f64::from(item.price.cents) / 100.0;
            let price_pts = (float_price * 0.2).ceil() as u64;
            total_pts += price_pts;
        }
    }

    if receipt.purchase_date.day() % 2 != 0 {
        total_pts += 6;
    }

    if receipt.purchase_time > time!(14:00) && receipt.purchase_time < time!(16:00) {
        total_pts += 10;
    }

    total_pts.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PointsResponse {
    pub points: u64,
}

#[get("/receipts/{id}/points")]
pub async fn get_points(path: web::Path<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();
    let Some(receipt) = data.connection.load_receipt(id).await else {
        return HttpResponse::NotFound().into();
    };
    let points = calculate_points(&receipt);
    HttpResponse::Ok().json(PointsResponse { points })
}
