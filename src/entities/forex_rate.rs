use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct DailyForexRate {
    pub rates: Vec<ForexRate>,
}

#[derive(Debug)]
pub struct ForexRate {
    pub selling: f64,
    pub billing: f64,
    pub currency: Currency,
}

impl ForexRate {
    pub fn new(currency: Currency, selling: f64, billing: f64) -> Self {
        Self {
            selling,
            billing,
            currency,
        }
    }
}

#[derive(Debug)]
pub enum Currency {
    Sdr,
    Euro,
    USDollar,
    UAEDirham,
    SwissFranc,
    SaudiRiyal,
    ChineseYuan,
    JapaneseYen,
    IndianRupee,
    DanishKroner,
    KuwaitiDinar,
    SwedishKroner,
    DjiboutiFranc,
    PoundSterling,
    KenyanShilling,
    CanadianDollar,
    NorwegianKroner,
    SouthAfricanRand,
    AustralianDollar,
}

#[derive(Debug)]
pub enum CurrencyParseError {
    UnrecognizedPattern,
}

impl Display for CurrencyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyParseError::UnrecognizedPattern => write!(f, "UnrecognizedPattern"),
        }
    }
}

impl FromStr for Currency {
    type Err = CurrencyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "sdr" => Ok(Currency::Sdr),
            "euro" => Ok(Currency::Euro),
            "us dollar" => Ok(Currency::USDollar),
            "uae dirham" => Ok(Currency::UAEDirham),
            "swiss franc" => Ok(Currency::SwissFranc),
            "saudi riyal" => Ok(Currency::SaudiRiyal),
            "japanese yen" => Ok(Currency::JapaneseYen),
            "chinese yuan" => Ok(Currency::ChineseYuan),
            "indian rupee" => Ok(Currency::IndianRupee),
            "kuwaiti dinar" => Ok(Currency::KuwaitiDinar),
            "danish kroner" => Ok(Currency::DanishKroner),
            "pound sterling" => Ok(Currency::PoundSterling),
            "swedish kroner" => Ok(Currency::SwedishKroner),
            "djibouti franc" => Ok(Currency::DjiboutiFranc),
            "canadian dollar" => Ok(Currency::CanadianDollar),
            "kenyan shilling" => Ok(Currency::KenyanShilling),
            "norwegian kroner" => Ok(Currency::NorwegianKroner),
            "australian dollar" => Ok(Currency::AustralianDollar),
            "south african rand" => Ok(Currency::SouthAfricanRand),
            _ => Err(CurrencyParseError::UnrecognizedPattern),
        }
    }
}
