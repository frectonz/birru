use scraper::{Html, Selector};

use crate::{
    entities::{Currency, DailyForexRate, ForexRate},
    traits::Parser,
};

pub struct HtmlParser;

impl Parser for HtmlParser {
    fn parse_daily_forex_rate(data: &String) -> DailyForexRate {
        let td_selector = Selector::parse("td").unwrap();
        let table_selector = Selector::parse("body > table > tbody > tr").unwrap();

        let document = Html::parse_document(data);
        let tables = document.select(&table_selector).collect::<Vec<_>>();

        let rates = tables
            .iter()
            .skip(1)
            .map(|trs| {
                let mut tds = trs.select(&td_selector).skip(1);

                let currency = tds.next().unwrap().inner_html();
                let buying = tds.next().unwrap().inner_html();
                let selling = tds.next().unwrap().inner_html();

                let currency = currency.split(";").collect::<Vec<_>>();
                let buying = buying.split(";").collect::<Vec<_>>();
                let selling = selling.split(";").collect::<Vec<_>>();

                let currency = currency.last().unwrap().trim();
                let buying = buying.last().unwrap().trim();
                let selling = selling.last().unwrap().trim();

                let currency = currency.parse::<Currency>().unwrap();
                let buying = buying.parse::<f64>().unwrap();
                let selling = selling.parse::<f64>().unwrap();

                ForexRate::new(currency, buying, selling)
            })
            .collect::<Vec<_>>();

        DailyForexRate { rates }
    }
}
