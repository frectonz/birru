use async_trait::async_trait;
use reqwest::{get, Error as ReqwestError};

use crate::ports::{DownloadError, Downloader};

const DAILY_FOREX_RATE: &str = "https://market.nbebank.com/market/banks/";

pub struct ReqwestDownloader;

#[async_trait]
impl Downloader for ReqwestDownloader {
    async fn download_daily_forex_rate() -> Result<String, DownloadError> {
        let res = get(DAILY_FOREX_RATE).await?;
        let text = res.text().await?;
        Ok(text)
    }
}

impl From<ReqwestError> for DownloadError {
    fn from(_: ReqwestError) -> Self {
        DownloadError::DownloadFailed
    }
}
