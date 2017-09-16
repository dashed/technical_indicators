// traits

struct DataPoint<T> {
    open: T,
    high: T,
    low: T,
    close: T,

    volume: T,
}

trait Chart<T> {
    fn get(&self, index: u64) -> DataPoint<T>;
}

// chart styles

struct Candlestick;

struct Renko;

// indicators

// ichimoku

struct Ichimoku<T> {
    chart: Box<Chart<T>>,
}

impl<T> Ichimoku<T> {
    fn new(
        chart: Box<Chart<T>>,

        // params
        conversion_line_period: u64,
        base_line_period: u64,
        span_b_period: u64,
        lagging_span_displacement: u64,
    ) -> Self {
        Ichimoku { chart: chart }
    }

    fn turning_line(index: u64) -> f64 {
        0.0
    }

    fn standard_line(index: u64) -> f64 {
        0.0
    }

    fn span_a(index: u64) -> f64 {
        0.0
    }

    fn span_b(index: u64) -> f64 {
        0.0
    }

    fn lagging_line(index: u64) -> f64 {
        0.0
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
