// local imports

use charts::{Chart, Source};

// Simple Moving Average (SMA)

pub struct SimpleMovingAverage<'chart> {
    chart: Box<&'chart Chart>,
    period_length: usize,
}

impl<'chart> SimpleMovingAverage<'chart> {
    pub fn new(chart: Box<&'chart Chart>, length: usize) -> Self {
        SimpleMovingAverage {
            chart: chart,
            period_length: length,
        }
    }

    pub fn get(&self, source_type: &Source, index: usize) -> Option<f64> {
        let mut total = 0.0;

        for get_index in index..(index + self.period_length) {
            match self.chart.get(get_index) {
                None => {
                    return None;
                }
                Some(data) => {
                    total += data.get(source_type);
                }
            }
        }

        Some(total / (self.period_length as f64))
    }

    pub fn open(&self, index: usize) -> Option<f64> {
        self.get(&Source::Open, index)
    }

    pub fn high(&self, index: usize) -> Option<f64> {
        self.get(&Source::High, index)
    }

    pub fn low(&self, index: usize) -> Option<f64> {
        self.get(&Source::Low, index)
    }

    pub fn close(&self, index: usize) -> Option<f64> {
        self.get(&Source::Close, index)
    }

    pub fn volume(&self, index: usize) -> Option<f64> {
        self.get(&Source::Volume, index)
    }
}
