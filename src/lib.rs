// traits

struct DataPoint {
    open: f64,
    high: f64,
    low: f64,
    close: f64,

    volume: f64,
}

trait Chart {
    fn get(&self, index: u64) -> DataPoint;
}

// chart styles

struct Candlestick;

struct Renko;

// indicators

// ichimoku

struct Ichimoku<'chart> {
    chart: Box<&'chart Chart>,
}

impl<'chart> Ichimoku<'chart> {
    fn new(
        // TODO: needs better approach
        chart: Box<&'chart Chart>,

        // params
        conversion_line_period: u64,
        base_line_period: u64,
        span_b_period: u64,
        lagging_span_displacement: u64,
    ) -> Self {
        Ichimoku { chart: chart }
    }

    fn turning_line(&self, index: u64) -> f64 {
        // TODO: implement
        self.chart.get(index).open
    }

    fn standard_line(&self, index: u64) -> f64 {
        // TODO: implement
        0.0
    }

    fn span_a(&self, index: u64) -> f64 {
        // TODO: implement
        0.0
    }

    fn span_b(&self, index: u64) -> f64 {
        // TODO: implement
        0.0
    }

    fn lagging_line(&self, index: u64) -> f64 {
        // TODO: implement
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
