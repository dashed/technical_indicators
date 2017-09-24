// local imports

use technical_indicators::charts::Chart;
use technical_indicators::charts::candlestick::CandleStick;

use TRADING_DATA;

// tests

#[test]
fn push_data_point() {
    let mut candlestick: CandleStick = CandleStick::new();

    match candlestick.get(0) {
        Some(_) => unreachable!(),
        None => {}
    };

    candlestick.push(&TRADING_DATA[0]);

    match candlestick.get(0) {
        Some(data_point) => {
            assert_eq!(data_point.open, TRADING_DATA[0].open);
        }
        None => unreachable!(),
    };
}
