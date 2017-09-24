// local imports

use technical_indicators::indicators::SimpleMovingAverage;
use technical_indicators::charts::{Chart, Source};
use technical_indicators::charts::candlestick::CandleStick;

use TRADING_DATA;

fn simple_moving_average(source_type: &Source, period: usize, index: usize) -> f64 {
    let end = TRADING_DATA.len();

    let mut sum = 0.0;
    for data in (&TRADING_DATA)[(end - period - index)..(end - index)].iter() {
        sum += data.get(source_type);
    }

    sum / (period as f64)
}

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

    let period = 50;
    let sma = SimpleMovingAverage::new(Box::new(&candlestick), period);

    for index in 0..(TRADING_DATA.len() - period) {
        match sma.open(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Open, period, index));
            }
            None => unreachable!(),
        }

        match sma.high(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::High, period, index));
            }
            None => unreachable!(),
        }

        match sma.low(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Low, period, index));
            }
            None => unreachable!(),
        }

        match sma.close(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Close, period, index));
            }
            None => unreachable!(),
        }

        match sma.volume(index) {
            Some(result) => {
                assert_approx_eq!(
                    result,
                    simple_moving_average(&Source::Volume, period, index)
                );
            }
            None => unreachable!(),
        }
    }

    match sma.open(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma.high(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }


    match sma.low(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma.close(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma.volume(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }
}
