use crate::entities::DailyForexRate;

pub trait Parser {
    fn parse_daily_forex_rate(data: &String) -> DailyForexRate;
}
