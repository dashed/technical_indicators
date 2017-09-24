
// Simple Moving Average (SMA)

pub struct SimpleMovingAverage<'source> {
    source: &'source [f64],
    period_length: usize,
}

impl<'source> SimpleMovingAverage<'source> {
    pub fn new(source: &'source [f64], length: usize) -> Self {
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
                    total += *data;
                }
            }
        }

        Some(total / (self.period_length as f64))
    }
}
