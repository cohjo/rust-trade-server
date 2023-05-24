mod constants;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use constants::{ALPACA_DATA_URL};
use reqwest::Client;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/stock/{ticker}")]
async fn get_stock(ticker: web::Path<String>) -> impl Responder {
    let key_id = std::env::var("ALPACA_API_KEY_ID").unwrap();
    let secret_key = std::env::var("ALPACA_API_SECRET_KEY").unwrap();
    let client = Client::new();
    let url = format!(
        "{}/stocks/trades/latest?symbols={}",
        ALPACA_DATA_URL, ticker
    );
    let resp = client
        .get(&url)
        .header("APCA-API-KEY-ID", key_id)
        .header("APCA-API-SECRET-KEY", secret_key)
        .send()
        .await;
    
    match resp {
        Ok(res) => {
            if res.status().is_success() {
                let body = res
                    .text()
                    .await
                    .unwrap_or_else(|_| String::from("Failed to read response body"));
                HttpResponse::Ok().body(body)
            } else {
                HttpResponse::InternalServerError().body("Failed to fetch stock")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Alpaca API"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_stock)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
