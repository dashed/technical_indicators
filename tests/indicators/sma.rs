// local imports

use technical_indicators::indicators::SimpleMovingAverage;
use technical_indicators::charts::Chart;
use technical_indicators::charts::candlestick::CandleStick;

use TRADING_DATA;

// tests

#[test]
fn push_get_data_points() {
    let mut candlestick = CandleStick::new();

    match candlestick.get(0) {
        Some(_) => unreachable!(),
        None => {}
    };

    for data in TRADING_DATA.iter() {
        candlestick.push(data);
    }

    let sma = SimpleMovingAverage::new(Box::new(&candlestick), 50);

    match sma.close(2) {
        Some(result) => {
            assert_eq!(result, 3791.7304);
        }
        None => unreachable!(),
    }
}
