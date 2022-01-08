use super::ForexRate;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct DailyForexRate(Vec<ForexRate>);

impl DailyForexRate {
    pub fn new(rates: Vec<ForexRate>) -> Self {
        Self(rates)
    }
}

impl Display for DailyForexRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0.iter().map(|fr| fr.to_string()).collect::<String>();
        write!(f, "{}", data)
    }
}
