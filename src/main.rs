use axum::{routing::get, Json, Router};
use birru::{
    adapters::{HtmlParser, ReqwestDownloader},
    domain::DailyForexRate,
    ports::{Downloader, Parser},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/", get(get_daily_forex_rate));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_daily_forex_rate() -> Json<DailyForexRate> {
    let data = ReqwestDownloader::download_daily_forex_rate()
        .await
        .unwrap();
    let rates = HtmlParser::parse_daily_forex_rate(data).await;
    println!("Parsed rates:\n{}", rates);
    Json(rates)
}
