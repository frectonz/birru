use async_trait::async_trait;

use std::{error::Error, fmt::Display};

pub const DAILY_FOREX_RATE: &str = "https://market.nbebank.com/market/banks/";

#[derive(Debug)]
pub enum DownloadError {
    FailedToGet,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "DownloadError"),
        }
    }
}

impl Error for DownloadError {}

#[async_trait]
pub trait Downloader {
    async fn download_daily_forex_rate() -> Result<String, DownloadError>;
}
