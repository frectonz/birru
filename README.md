# Birru

A REST server that returns data related to Ethiopian forex exchange rates.

## Usage

Running the server

```bash
$ cargo run
Listening on http://0.0.0.0:3000
```

Testing with [httpie](https://httpie.io/).

```bash
$ http localhost:3000
```

## Sample Response

```json
[
  {
    "selling": 13.3139,
    "buying": 13.0528,
    "currency": "UAEDirham"
  },
  {
    "selling": 36.6317,
    "buying": 35.9134,
    "currency": "CanadianDollar"
  },
  {
    "selling": 4.637,
    "buying": 4.5461,
    "currency": "SwedishKroner"
  },
  {
    "selling": 71.0745,
    "buying": 69.6809,
    "currency": "Sdr"
  },
  {
    "selling": 6.8158,
    "buying": 6.6822,
    "currency": "DanishKroner"
  },
  {
    "selling": 167.5669,
    "buying": 164.2813,
    "currency": "KuwaitiDinar"
  },
  {
    "selling": 54.4415,
    "buying": 53.374,
    "currency": "SwissFranc"
  },
  {
    "selling": 0.3002,
    "buying": 0.2943,
    "currency": "DjiboutiFranc"
  },
  {
    "selling": 0.6618,
    "buying": 0.6488,
    "currency": "IndianRupee"
  },
  {
    "selling": 0.4426,
    "buying": 0.4339,
    "currency": "KenyanShilling"
  },
  {
    "selling": 3.101,
    "buying": 3.0402,
    "currency": "SouthAfricanRand"
  },
  {
    "selling": 54.0367,
    "buying": 52.9772,
    "currency": "USDollar"
  },
  {
    "selling": 6.8391,
    "buying": 6.705,
    "currency": "ChineseYuan"
  },
  {
    "selling": 13.0096,
    "buying": 12.7545,
    "currency": "SaudiRiyal"
  },
  {
    "selling": 0.3501,
    "buying": 0.3432,
    "currency": "JapaneseYen"
  },
  {
    "selling": 61.3172,
    "buying": 60.1149,
    "currency": "PoundSterling"
  },
  {
    "selling": 32.7211,
    "buying": 32.0795,
    "currency": "AustralianDollar"
  },
  {
    "selling": 56.0145,
    "buying": 54.9162,
    "currency": "Euro"
  },
  {
    "selling": 4.8497,
    "buying": 4.7546,
    "currency": "NorwegianKroner"
  }
]
```

Supported currencies are:

- UAEDirham
- CanadianDollar
- SwedishKroner
- Sdr
- DanishKroner
- KuwaitiDinar
- SwissFranc
- DjiboutiFranc
- IndianRupee
- KenyanShilling
- SouthAfricanRand
- USDollar
- ChineseYuan
- SaudiRiyal
- JapaneseYen
- PoundSterling
- AustralianDollar
- Euro
- NorwegianKroner
