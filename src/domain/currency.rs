use serde::Serialize;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Serialize)]
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

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Currency::*;
        match self {
            Sdr => write!(f, "SDR"),
            Euro => write!(f, "Euro"),
            USDollar => write!(f, "US Dollar"),
            UAEDirham => write!(f, "UAE Dirham"),
            SwissFranc => write!(f, "Swiss Franc"),
            SaudiRiyal => write!(f, "Saudi Riyal"),
            ChineseYuan => write!(f, "Chinese Yuan"),
            JapaneseYen => write!(f, "Japanese Yen"),
            IndianRupee => write!(f, "Indian Rupee"),
            DanishKroner => write!(f, "Danish Kroner"),
            KuwaitiDinar => write!(f, "Kuwaiti Dinar"),
            SwedishKroner => write!(f, "Swedish Kroner"),
            DjiboutiFranc => write!(f, "Djibouti Franc"),
            PoundSterling => write!(f, "Pound Sterling"),
            KenyanShilling => write!(f, "Kenyan Shilling"),
            CanadianDollar => write!(f, "Canadian Dollar"),
            NorwegianKroner => write!(f, "Norwegian Kroner"),
            AustralianDollar => write!(f, "Australian Dollar"),
            SouthAfricanRand => write!(f, "South African Rand"),
        }
    }
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
