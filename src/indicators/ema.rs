use charts::SourceSeries;
use indicators::sma::SimpleMovingAverage;

// Exponential Moving Average (EMA)

pub struct ExponentialMovingAverage<'source> {
    source: SourceSeries<'source>,

    // 2 / (length + 1)
    alpha: f64,

    // params
    length: usize,
}

impl<'source> ExponentialMovingAverage<'source> {
    pub fn new(source: SourceSeries<'source>, length: usize) -> Self {
        let alpha = 2.0 / ((length as f64) + 1.0);

        ExponentialMovingAverage {
            source: source,
            alpha: alpha,
            length: length,
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        // EMA = alpha * data + (1 - alpha) * EMA[1], where alpha = 2 / (length + 1)

        let last_index: i64 = (self.source.len() as i64) - 1;
        if last_index < 0 {
            return None;
        }

        // ensure the number of data points between index and last_index is at
        // least self.length
        let num_of_data_points_between_current_and_last = last_index - (index as i64) + 1;
        if num_of_data_points_between_current_and_last < (self.length as i64) {
            return None;
        }

        if num_of_data_points_between_current_and_last == (self.length as i64) {
            // set the first EMA point to be the SMA of the last self.length
            // data points

            let sma = SimpleMovingAverage::new(self.source.clone(), self.length);
            return sma.get(index);
        }

        let current_value = match self.source.get(index) {
            None => {
                return None;
            }
            Some(current_value) => current_value,
        };

        let prev_value = match self.get(index + 1) {
            None => 0.0,
            Some(prev_value) => prev_value,
        };

        let ema_value = self.alpha * current_value + (1.0 - self.alpha) * prev_value;

        Some(ema_value)
    }
}
