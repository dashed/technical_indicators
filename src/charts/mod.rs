#[derive(Clone)]
pub enum Source {
    Open,
    High,
    Low,
    Close,
    Volume,
}

impl AsRef<Source> for Source {
    fn as_ref(&self) -> &Source {
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl DataPoint {
    pub fn get<T: AsRef<Source>>(&self, source_type: T) -> f64 {
        match *source_type.as_ref() {
            Source::Open => self.open,
            Source::High => self.high,
            Source::Low => self.low,
            Source::Close => self.close,
            Source::Volume => self.volume,
        }
    }
}

pub trait Chart {
    fn get(&self, index: usize) -> Option<&DataPoint>;
    fn push(&mut self, data_point: &DataPoint);
}

pub mod candlestick;
