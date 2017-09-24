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
    let sma_open = SimpleMovingAverage::new(candlestick.open(), period);
    let sma_high = SimpleMovingAverage::new(candlestick.high(), period);
    let sma_low = SimpleMovingAverage::new(candlestick.low(), period);
    let sma_close = SimpleMovingAverage::new(candlestick.close(), period);
    let sma_volume = SimpleMovingAverage::new(candlestick.volume(), period);

    for index in 0..(TRADING_DATA.len() - period) {
        match sma_open.get(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Open, period, index));
            }
            None => unreachable!(),
        }

        match sma_high.get(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::High, period, index));
            }
            None => unreachable!(),
        }

        match sma_low.get(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Low, period, index));
            }
            None => unreachable!(),
        }

        match sma_close.get(index) {
            Some(result) => {
                assert_approx_eq!(result, simple_moving_average(&Source::Close, period, index));
            }
            None => unreachable!(),
        }

        match sma_volume.get(index) {
            Some(result) => {
                assert_approx_eq!(
                    result,
                    simple_moving_average(&Source::Volume, period, index)
                );
            }
            None => unreachable!(),
        }
    }

    // expect non since there aren't enough data points to build a SMA of given period size

    match sma_open.get(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma_high.get(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma_low.get(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma_close.get(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }

    match sma_volume.get(TRADING_DATA.len() - period + 1) {
        Some(_) => unreachable!(),
        None => {}
    }
}
