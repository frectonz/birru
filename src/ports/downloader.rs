use async_trait::async_trait;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as IOResult},
};

#[async_trait]
pub trait Downloader {
    async fn download_daily_forex_rate() -> Result<String, DownloadError>;
}

#[derive(Debug)]
pub enum DownloadError {
    DownloadFailed,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> IOResult {
        use DownloadError::*;
        match self {
            DownloadFailed => write!(f, "DownloadFailed"),
        }
    }
}

impl Error for DownloadError {}
