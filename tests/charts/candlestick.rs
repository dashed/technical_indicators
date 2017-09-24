// local imports

use technical_indicators::charts::Chart;
use technical_indicators::charts::candlestick::CandleStick;

use TRADING_DATA;

// tests

#[test]
fn push_get_data_points() {
    let mut candlestick: CandleStick = CandleStick::new();

    match candlestick.get(0) {
        Some(_) => unreachable!(),
        None => {}
    };

    for data in TRADING_DATA.iter() {
        candlestick.push(data);
    }

    for index in 0..TRADING_DATA.len() {
        match candlestick.get(index) {
            Some(data_point) => {
                let arr_index = TRADING_DATA.len() - 1 - index;
                assert_eq!(*data_point, TRADING_DATA[arr_index]);
            }
            None => unreachable!(),
        };
    }

    match candlestick.get(TRADING_DATA.len()) {
        Some(_) => unreachable!(),
        None => {}
    }
}
