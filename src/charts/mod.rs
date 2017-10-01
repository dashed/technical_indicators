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

pub struct SourceSeries<'chart> {
    chart: Box<&'chart Chart>,
    series_type: Source,
    offset: usize,
}

impl<'chart> SourceSeries<'chart> {
    fn new(chart: Box<&'chart Chart>, series_type: Source) -> Self {
        SourceSeries {
            chart: chart,
            series_type: series_type,
            offset: 0,
        }
    }

    pub fn offset(&self, index: usize) -> Self {
        SourceSeries {
            chart: self.chart.clone(),
            series_type: self.series_type.clone(),
            offset: index,
        }
    }

    /// Get the value of this `SourceSeries` at `index`.
    pub fn get(&self, index: usize) -> Option<f64> {
        match self.chart.get(index) {
            None => None,
            Some(data_point) => Some(data_point.get(&self.series_type)),
        }
    }
}

pub trait Chart {
    fn get(&self, index: usize) -> Option<&DataPoint>;
    fn push(&mut self, data_point: &DataPoint);

    fn open(&self) -> SourceSeries;
    fn high(&self) -> SourceSeries;
    fn low(&self) -> SourceSeries;
    fn close(&self) -> SourceSeries;
    fn volume(&self) -> SourceSeries;
}

pub mod candlestick;
