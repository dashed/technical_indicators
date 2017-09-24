// 3rd-party crates

extern crate json;
#[macro_use]
extern crate lazy_static;

// local crates

extern crate technical_indicators;

// local imports

use technical_indicators::charts::DataPoint;

// static

lazy_static! {
    static ref TRADING_DATA: Vec<DataPoint> = {

        use json::JsonValue;

        let raw_data = include_str!("data.json");

        let parsed = json::parse(raw_data).unwrap();

        match parsed["result"]["14400"] {
            JsonValue::Array(ref arr) => {
                arr.clone().iter_mut().map(|item| {

                    // NOTE: [ CloseTime, OpenPrice, HighPrice, LowPrice, ClosePrice, Volume ]
                    DataPoint {
                        open: item[1].as_f64().unwrap(),
                        high: item[2].as_f64().unwrap(),
                        low: item[3].as_f64().unwrap(),
                        close: item[4].as_f64().unwrap(),
                        volume: item[5].as_f64().unwrap(),
                    }
                }).collect()
            },
            _ => unreachable!()
        }
    };
}

// tests

pub mod charts;
pub mod indicators;
