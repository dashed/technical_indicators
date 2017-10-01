use charts::SourceSeries;

// Simple Moving Average (SMA)

pub struct SimpleMovingAverage<'source> {
    source: SourceSeries<'source>,

    // params
    period_length: usize,
}

impl<'source> SimpleMovingAverage<'source> {
    pub fn new(source: SourceSeries<'source>, length: usize) -> Self {
        SimpleMovingAverage {
            source: source,
            period_length: length,
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        let mut total = 0.0;

        for get_index in index..(index + self.period_length) {
            match self.source.get(get_index) {
                None => {
                    return None;
                }
                Some(data) => {
                    total += data;
                }
            }
        }

        Some(total / (self.period_length as f64))
    }
}
