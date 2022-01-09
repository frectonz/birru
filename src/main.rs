use axum::{routing::get, Json, Router};
use birru::{
    adapters::{HtmlParser, ReqwestDownloader},
    domain::DailyForexRate,
    ports::{Downloader, Parser},
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port = get_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let app = Router::new().route("/", get(get_daily_forex_rate));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Listening on http://{}", addr);
}

async fn get_daily_forex_rate() -> Json<DailyForexRate> {
    let data = ReqwestDownloader::download_daily_forex_rate()
        .await
        .unwrap();
    let rates = HtmlParser::parse_daily_forex_rate(data).await;
    println!("Parsed rates:\n{}", rates);
    Json(rates)
}

fn get_port() -> u16 {
    env::var("PORT")
        .map(|p| p.parse::<_>().expect("Failed to parse port"))
        .unwrap_or(3000)
}
