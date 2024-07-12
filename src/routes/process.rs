use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{data::Receipt, AppState};

/// Response sent by the process service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProcessReceiptResponse {
    pub id: Uuid,
}

/// Send receipt data for a new receipt to the database.
#[post("/receipts/process")]
pub async fn process_receipt(
    web::Json(receipt): web::Json<Receipt>,
    data: web::Data<AppState>,
) -> HttpResponse {
    match data.connection.store_receipt(receipt).await {
        Some(id) => HttpResponse::Ok().json(ProcessReceiptResponse { id }),
        None => HttpResponse::BadRequest().into(),
    }
}
