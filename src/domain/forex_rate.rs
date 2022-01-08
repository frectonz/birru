use super::Currency;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ForexRate {
    selling: f64,
    buying: f64,
    currency: Currency,
}

impl ForexRate {
    pub fn new(currency: Currency, buying: f64, selling: f64) -> Self {
        Self {
            selling,
            buying,
            currency,
        }
    }
}

impl Display for ForexRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.currency, self.buying, self.selling)
    }
}
