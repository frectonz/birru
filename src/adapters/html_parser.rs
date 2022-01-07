use scraper::{ElementRef, Html, Selector};

use crate::{
    domain::{Currency, DailyForexRate, ForexRate},
    ports::{Parser, ParserError},
};

pub struct HtmlParser;

impl Parser for HtmlParser {
    fn parse_daily_forex_rate(data: String) -> DailyForexRate {
        let table_selector =
            Selector::parse("body > table > tbody > tr").expect("Failed to parse <table> selector");

        let document = Html::parse_document(&data);
        let tables = document.select(&table_selector).collect::<Vec<_>>();

        let rates = tables
            .iter()
            .skip(1)
            .map(parse_forex_rate)
            .collect::<Vec<_>>();

        DailyForexRate::new(rates)
    }
}

fn parse_forex_rate(trs: &ElementRef) -> Result<ForexRate, ParserError> {
    let td_selector = Selector::parse("td").expect("Failed to parse <td> selector");

    let mut tds = trs.select(&td_selector).skip(1);

    let currency = tds
        .next()
        .map(|c| c.inner_html())
        .ok_or(ParserError::NoCurrencyValue)?;
    let buying = tds
        .next()
        .map(|b| b.inner_html())
        .ok_or(ParserError::NoBuyingValue)?;
    let selling = tds
        .next()
        .map(|b| b.inner_html())
        .ok_or(ParserError::NoSellingValue)?;

    let currency = currency
        .split(";")
        .last()
        .map(|c| c.trim())
        .ok_or(ParserError::CurrencyParseFailed)?;
    let buying = buying
        .split(";")
        .last()
        .map(|b| b.trim())
        .ok_or(ParserError::BuyingPriceParseFailed)?;
    let selling = selling
        .split(";")
        .last()
        .map(str::trim)
        .ok_or(ParserError::SellingPriceParseFailed)?;

    let currency = currency.parse::<Currency>()?;
    let buying = buying
        .parse::<f64>()
        .or(Err(ParserError::BuyingPriceParseFailed))?;
    let selling = selling
        .parse::<f64>()
        .or(Err(ParserError::SellingPriceParseFailed))?;

    Ok(ForexRate::new(currency, buying, selling))
}
