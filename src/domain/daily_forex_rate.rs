use super::ForexRate;
use crate::ports::ParserError;

#[derive(Debug)]
pub struct DailyForexRate {
    _rates: Vec<Result<ForexRate, ParserError>>,
}

impl DailyForexRate {
    pub fn new(rates: Vec<Result<ForexRate, ParserError>>) -> Self {
        Self { _rates: rates }
    }
}
