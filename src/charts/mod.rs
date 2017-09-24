
#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,

    pub volume: f64,
}

pub trait Chart {
    fn get(&self, index: usize) -> Option<&DataPoint>;
    fn push(&mut self, data_point: &DataPoint);
}

pub mod candlestick;
