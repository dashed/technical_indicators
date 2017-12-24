// 3rd-party crates

#[macro_use]
extern crate assert_approx_eq;
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

        // 14400 seconds := 4 hours
        match parsed["result"]["14400"] {
            JsonValue::Array(ref arr) => {
                arr.clone().iter_mut().map(|item| {

                    // NOTE: [ CloseTime, OpenPrice, HighPrice, LowPrice, ClosePrice, Volume ]

                    let open = item[1].as_f64().unwrap();
                    let high = item[2].as_f64().unwrap();
                    let low = item[3].as_f64().unwrap();
                    let close = item[4].as_f64().unwrap();

                    assert!(low <= high);
                    assert!(open <= high);
                    assert!(low <= open);
                    assert!(close <= high);
                    assert!(low <= close);

                    DataPoint {
                        open: open,
                        high: high,
                        low: low,
                        close: close,
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
