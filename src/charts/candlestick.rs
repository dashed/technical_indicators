use charts::{Chart, DataPoint};
use std::collections::VecDeque;

// candlestick chart
// Ref: https://en.wikipedia.org/wiki/Open-high-low-close_chart

pub struct CandleStick {
    candles: VecDeque<DataPoint>,
}

impl CandleStick {
    pub fn new() -> Self {
        CandleStick { candles: VecDeque::new() }
    }
}

impl Chart for CandleStick {
    fn as_chart(&self) -> &Chart {
        self
    }

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
        self.candles.push_back(data_point.clone());
    }

    fn pop_front(&mut self) {
        self.candles.pop_front();
    }

    fn len(&self) -> usize {
        self.candles.len()
    }
}
