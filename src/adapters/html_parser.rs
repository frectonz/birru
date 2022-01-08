use crate::{
    domain::{Currency, DailyForexRate, ForexRate},
    ports::{Parser, ParserError},
};
use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};

const TD_SELECTOR: &str = "td";
const TABLE_SELECTOR: &str = "body > table > tbody > tr";
const SEPARATOR: &str = ";";

pub struct HtmlParser;

#[async_trait]
impl Parser for HtmlParser {
    async fn parse_daily_forex_rate(data: String) -> DailyForexRate {
        let handle = tokio::spawn(async move {
            let table_selector =
                Selector::parse(TABLE_SELECTOR).expect("Failed to parse <table> selector");
            let document = Html::parse_document(&data);
            let tables = document.select(&table_selector).collect::<Vec<_>>();

            let rates = tables
                .iter()
                .skip(1)
                .map(parse_forex_rate)
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect::<Vec<_>>();

            DailyForexRate::new(rates)
        });

        handle.await.expect("Failed to `join` async handle")
    }
}

fn parse_forex_rate(trs: &ElementRef) -> Result<ForexRate, ParserError> {
    use ParserError::*;
    let td_selector = Selector::parse(TD_SELECTOR).expect("Failed to parse <td> selector");

    let mut tds = trs.select(&td_selector).skip(1);

    let currency = tds.next().map(|c| c.inner_html()).ok_or(NoCurrencyValue)?;
    let buying = tds.next().map(|b| b.inner_html()).ok_or(NoBuyingValue)?;
    let selling = tds.next().map(|b| b.inner_html()).ok_or(NoSellingValue)?;

    let currency = currency
        .split(SEPARATOR)
        .last()
        .map(str::trim)
        .ok_or(CurrencyParseFailed)?;
    let buying = buying
        .split(SEPARATOR)
        .last()
        .map(str::trim)
        .ok_or(BuyingPriceParseFailed)?;
    let selling = selling
        .split(SEPARATOR)
        .last()
        .map(str::trim)
        .ok_or(SellingPriceParseFailed)?;

    let currency = currency.parse::<Currency>().or(Err(CurrencyParseFailed))?;
    let buying = buying.parse::<f64>().or(Err(BuyingPriceParseFailed))?;
    let selling = selling.parse::<f64>().or(Err(SellingPriceParseFailed))?;

    Ok(ForexRate::new(currency, buying, selling))
}
