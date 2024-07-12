mod points;
mod process;

// Re-export the routes
pub use points::get_points;
pub use process::process_receipt;

#[cfg(test)]
mod tests {
    use actix_web::{body, http::header::ContentType, test, web::Data, App};
    use serde::Deserialize;

    use crate::{
        db::Connection,
        routes::{points::PointsResponse, process::ProcessReceiptResponse},
        AppState,
    };

    use super::*;

    /// Takes data for a receipt, sends it to the server, and gets the points total for that receipt.
    async fn run_full_trip(receipt_json: &'static [u8], expected_pts: u64) {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    connection: Connection::new(),
                }))
                .service(process_receipt)
                .service(get_points),
        )
        .await;

        // put a receipt in
        let process_req = test::TestRequest::post()
            .uri("/receipts/process")
            .insert_header(ContentType::json())
            .set_payload(receipt_json)
            .to_request();
        let process_resp = test::call_service(&app, process_req).await;
        assert!(process_resp.status().is_success());

        let process_body = body::to_bytes(process_resp.into_body()).await.unwrap();
        let mut deserializer = serde_json::Deserializer::from_slice(&process_body);
        let ProcessReceiptResponse { id } = Deserialize::deserialize(&mut deserializer).unwrap();

        // get its points
        let points_req = test::TestRequest::get()
            .uri(&format!("/receipts/{id}/points"))
            .to_request();
        let points_resp = test::call_service(&app, points_req).await;
        assert!(points_resp.status().is_success());

        let points_body = body::to_bytes(points_resp.into_body()).await.unwrap();
        let mut deserializer = serde_json::Deserializer::from_slice(&points_body);
        let PointsResponse { points } = Deserialize::deserialize(&mut deserializer).unwrap();
        assert_eq!(points, expected_pts);
    }

    #[actix_web::test]
    async fn simple_receipt() {
        run_full_trip(
            br#"
                {
                    "retailer": "Target",
                    "purchaseDate": "2022-01-02",
                    "purchaseTime": "13:13",
                    "total": "1.25",
                    "items": [
                        { "shortDescription": "Pepsi - 12-oz", "price": "1.25" }
                    ]
                }
            "#,
            31,
        )
        .await;
    }

    #[actix_web::test]
    async fn example_1() {
        run_full_trip(
            br#"
            {
                "retailer": "Target",
                "purchaseDate": "2022-01-01",
                "purchaseTime": "13:01",
                "items": [
                {
                    "shortDescription": "Mountain Dew 12PK",
                    "price": "6.49"
                },{
                    "shortDescription": "Emils Cheese Pizza",
                    "price": "12.25"
                },{
                    "shortDescription": "Knorr Creamy Chicken",
                    "price": "1.26"
                },{
                    "shortDescription": "Doritos Nacho Cheese",
                    "price": "3.35"
                },{
                    "shortDescription": "   Klarbrunn 12-PK 12 FL OZ  ",
                    "price": "12.00"
                }
                ],
                "total": "35.35"
            }"#,
            28,
        )
        .await;
    }

    #[actix_web::test]
    async fn example_2() {
        run_full_trip(
            br#"
            {
                "retailer": "M&M Corner Market",
                "purchaseDate": "2022-03-20",
                "purchaseTime": "14:33",
                "items": [
                {
                    "shortDescription": "Gatorade",
                    "price": "2.25"
                },{
                    "shortDescription": "Gatorade",
                    "price": "2.25"
                },{
                    "shortDescription": "Gatorade",
                    "price": "2.25"
                },{
                    "shortDescription": "Gatorade",
                    "price": "2.25"
                }
                ],
                "total": "9.00"
            }"#,
            109,
        )
        .await;
    }
}
