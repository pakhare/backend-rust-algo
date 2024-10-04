// src/mock_api.rs
use actix_web::{web, App, HttpServer, HttpResponse};
use serde_json::json;

async fn mock_trade() -> HttpResponse {
    let mock_data = vec![
        json!({
            "id": 1,
            "symbol": "AAPL",
            "price": 150.0,
            "quantity": 10,
        }),
        json!({
            "id": 2,
            "symbol": "GOOGL",
            "price": 2800.0,
            "quantity": 5,
        }),
    ];

    HttpResponse::Ok().json(mock_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/stock_prices", web::get().to(mock_trade)) // Mock endpoint
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
