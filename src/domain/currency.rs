use std::{fmt::Display, str::FromStr};

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

impl FromStr for Currency {
    type Err = CurrencyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Currency::*;

        match s.trim().to_lowercase().as_str() {
            "sdr" => Ok(Sdr),
            "euro" => Ok(Euro),
            "us dollar" => Ok(USDollar),
            "uae dirham" => Ok(UAEDirham),
            "swiss franc" => Ok(SwissFranc),
            "saudi riyal" => Ok(SaudiRiyal),
            "japanese yen" => Ok(JapaneseYen),
            "chinese yuan" => Ok(ChineseYuan),
            "indian rupee" => Ok(IndianRupee),
            "kuwaiti dinar" => Ok(KuwaitiDinar),
            "danish kroner" => Ok(DanishKroner),
            "pound sterling" => Ok(PoundSterling),
            "swedish kroner" => Ok(SwedishKroner),
            "djibouti franc" => Ok(DjiboutiFranc),
            "canadian dollar" => Ok(CanadianDollar),
            "kenyan shilling" => Ok(KenyanShilling),
            "norwegian kroner" => Ok(NorwegianKroner),
            "australian dollar" => Ok(AustralianDollar),
            "south african rand" => Ok(SouthAfricanRand),
            _ => Err(CurrencyParseError::UnrecognizedPattern),
        }
    }
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
