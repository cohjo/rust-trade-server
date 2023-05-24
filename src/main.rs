mod constants;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use constants::ALPACA_DATA_URL;
use reqwest::Client;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(serde::Deserialize)]
struct LatestStockQueryParams {
    ticker: String,
}

#[get("/stocks/bars/latest")]
async fn get_stock(stock: web::Query<LatestStockQueryParams>) -> impl Responder {
    let key_id = std::env::var("ALPACA_API_KEY_ID").unwrap();
    let secret_key = std::env::var("ALPACA_API_SECRET_KEY").unwrap();
    let client = Client::new();
    let url = format!(
        "{}/stocks/bars/latest?symbols={}",
        ALPACA_DATA_URL, &stock.ticker
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
                HttpResponse::InternalServerError().body("Failed to fetch latest stock bar")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Alpaca API"),
    }
}

#[derive(serde::Deserialize)]
struct StockQueryParams {
    ticker: String,
    from: String,
    to: String,
    agg: String,
}

#[get("/stocks/bars/historical")]
async fn get_stock_history(stock: web::Query<StockQueryParams>) -> impl Responder {
    let key_id = std::env::var("ALPACA_API_KEY_ID").unwrap();
    let secret_key = std::env::var("ALPACA_API_SECRET_KEY").unwrap();

    let ticker = &stock.ticker;
    let from = &stock.from;
    let to = &stock.to;
    let agg = &stock.agg;
    const LIM: usize = 1000;

    let client = Client::new();
    let url = format!(
        "{}/stocks/bars?symbols={}&start={}&end={}&timeframe={}&limit={}",
        ALPACA_DATA_URL, ticker, from, to, agg, LIM
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
                HttpResponse::InternalServerError().body("Failed to fetch historical stock bars")
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
            .service(get_stock_history)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
