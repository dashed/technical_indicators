use charts::SourceSeries;

// Weighted Moving Average (WMA)

pub struct WeightedMovingAverage<'source> {
    source: SourceSeries<'source>,

    // params
    length: usize,
}

impl<'source> WeightedMovingAverage<'source> {
    pub fn new(source: SourceSeries<'source>, length: usize) -> Self {
        WeightedMovingAverage {
            source: source,
            length: length,
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {

        if self.length <= 0 {
            return None;
        }

        let offset_source = self.source.offset(index);

        let mut norm = 0.0;
        let mut total = 0.0;

        for normalized_index  in 0..self.length {
            match offset_source.get(normalized_index) {
                None => {
                    return None;
                }
                Some(data) => {
                    let weight = (self.length - normalized_index) * self.length;
                    let weight = weight as f64;
                    norm = norm + weight;
                    total = total + data * weight;
                }
            }
        }

        Some(total / norm)
    }
}
