use async_trait::async_trait;

use crate::traits::{DownloadError, Downloader, DAILY_FOREX_RATE};

pub struct ReqwestDownloader;

#[async_trait]
impl Downloader for ReqwestDownloader {
    async fn download_daily_forex_rate() -> Result<String, DownloadError> {
        let res = reqwest::get(DAILY_FOREX_RATE).await?;
        let text = res.text().await?;
        Ok(text)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(_: reqwest::Error) -> Self {
        DownloadError::FailedToGet
    }
}
