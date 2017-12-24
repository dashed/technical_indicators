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

#[derive(Clone)]
pub struct SourceSeries<'chart> {
    chart: &'chart Chart,
    series_type: Source,
    offset: usize,
}

impl<'chart> SourceSeries<'chart> {
    pub fn new(chart: &'chart Chart, series_type: Source) -> Self {
        SourceSeries {
            chart: chart,
            series_type: series_type,
            offset: 0,
        }
    }

    /// Generate new `SourceSeries` by offsetting by `length` data points.
    pub fn offset(&self, length: usize) -> Self {
        SourceSeries {
            chart: self.chart.clone(),
            series_type: self.series_type.clone(),
            offset: length,
        }
    }

    /// Get the value of this `SourceSeries` at `index`.
    /// If there is `self.offset`, it is added to `index`.
    pub fn get(&self, index: usize) -> Option<f64> {
        match self.chart.get(index + self.offset) {
            None => None,
            Some(data_point) => Some(data_point.get(&self.series_type)),
        }
    }

    pub fn len(&self) -> usize {
        self.chart.len()
    }
}

pub trait Chart {
    fn as_chart(&self) -> &Chart;

    /// Get value of this `Chart` at `index`.
    fn get(&self, index: usize) -> Option<&DataPoint>;

    /// Get number of data points in this chart.
    fn len(&self) -> usize;

    /// Add newest data point to the chart.
    fn push(&mut self, data_point: &DataPoint);

    /// Remove oldest data point from the chart.
    fn pop_front(&mut self);

    fn open<'chart>(&'chart self) -> SourceSeries {
        let chart: &'chart Chart = self.as_chart();
        SourceSeries::new(chart, Source::Open)
    }

    fn high<'chart>(&'chart self) -> SourceSeries {
        let chart: &'chart Chart = self.as_chart();
        SourceSeries::new(chart, Source::High)
    }

    fn low<'chart>(&'chart self) -> SourceSeries {
        let chart: &'chart Chart = self.as_chart();
        SourceSeries::new(chart, Source::Low)
    }

    fn close<'chart>(&'chart self) -> SourceSeries {
        let chart: &'chart Chart = self.as_chart();
        SourceSeries::new(chart, Source::Close)
    }

    fn volume<'chart>(&'chart self) -> SourceSeries {
        let chart: &'chart Chart = self.as_chart();
        SourceSeries::new(chart, Source::Volume)
    }
}

pub mod utils;

// types of charts

pub mod candlestick;
