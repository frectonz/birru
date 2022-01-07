use super::Currency;

#[derive(Debug)]
pub struct ForexRate {
    _selling: f64,
    _buying: f64,
    _currency: Currency,
}

impl ForexRate {
    pub fn new(currency: Currency, buying: f64, selling: f64) -> Self {
        Self {
            _selling: selling,
            _buying: buying,
            _currency: currency,
        }
    }
}
