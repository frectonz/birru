use crate::domain::DailyForexRate;

#[derive(Debug)]
pub enum ParserError {
    EmptyDocument,
    NoBuyingValue,
    NoSellingValue,
    NoCurrencyValue,
    CurrencyParseFailed,
    BuyingPriceParseFailed,
    SellingPriceParseFailed,
}

pub trait Parser {
    fn parse_daily_forex_rate(data: String) -> DailyForexRate;
}
