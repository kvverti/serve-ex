use std::io;

use actix_web::{web, App, HttpServer};
use db::Connection;

mod data;
mod db;
mod routes;

#[derive(Debug)]
struct AppState {
    connection: Connection,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // for simplicity, we'll create a "connection" to our "database" here
    let db_conn = Connection::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                connection: db_conn.clone(),
            }))
            .service(routes::get_points)
            .service(routes::process_receipt)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
