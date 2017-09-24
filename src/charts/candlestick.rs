use charts::DataPoint;
use charts::Chart;

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

        if index >= len {
            return None;
        }

        let normalized_index = (len - 1) - index;

        self.candles.get(normalized_index)
    }

    fn push(&mut self, data_point: &DataPoint) {
        self.candles.push(data_point.clone());
    }
}
