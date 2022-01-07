use birru::{
    adapters::{HtmlParser, ReqwestDownloader},
    ports::{Downloader, Parser},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = ReqwestDownloader::download_daily_forex_rate().await?;
    let rates = HtmlParser::parse_daily_forex_rate(data);

    dbg!(rates);

    Ok(())
}
