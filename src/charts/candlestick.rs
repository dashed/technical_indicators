use charts::{Chart, DataPoint, Source, SourceSeries};

// candlestick chart
// Ref: https://en.wikipedia.org/wiki/Open-high-low-close_chart

pub struct CandleStick {
    candles: Vec<DataPoint>,
}

impl CandleStick {
    pub fn new() -> Self {
        CandleStick { candles: vec![] }
    }
}

impl Chart for CandleStick {
    fn get(&self, index: usize) -> Option<&DataPoint> {
        let len = self.candles.len();

        if len <= 0 {
            return None;
        }

        if index >= len {
            return None;
        }

        let len_index = len - 1;
        let normalized_index = len_index - index;

        self.candles.get(normalized_index)
    }

    fn push(&mut self, data_point: &DataPoint) {
        self.candles.push(data_point.clone());
    }

    fn open(&self) -> SourceSeries {
        SourceSeries::new(Box::new(self), Source::Open)
    }

    fn high(&self) -> SourceSeries {
        SourceSeries::new(Box::new(self), Source::High)
    }

    fn low(&self) -> SourceSeries {
        SourceSeries::new(Box::new(self), Source::Low)
    }

    fn close(&self) -> SourceSeries {
        SourceSeries::new(Box::new(self), Source::Close)
    }

    fn volume(&self) -> SourceSeries {
        SourceSeries::new(Box::new(self), Source::Volume)
    }
}
