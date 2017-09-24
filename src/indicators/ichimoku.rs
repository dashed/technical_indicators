// local imports

use charts::Chart;

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
