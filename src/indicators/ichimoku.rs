// local imports

use charts::Chart;
use charts::SourceSeries;
use charts::utils::{highest, lowest};

// Ichimoku Kinkō Hyō

pub struct Ichimoku<'chart> {
    chart: &'chart Chart,

    // params
    turning_line_period: usize,
    standard_line_period: usize,
    span_b_period: usize,
    lagging_span_displacement: usize,
}

impl<'chart> Ichimoku<'chart> {
    fn new(
        chart: &'chart Chart,

        // params
        turning_line_period: usize,
        standard_line_period: usize,
        span_b_period: usize,
        lagging_span_displacement: usize,
    ) -> Self {
        Ichimoku {
            chart: chart,

            // params
            turning_line_period: turning_line_period,
            standard_line_period: standard_line_period,
            span_b_period: span_b_period,
            lagging_span_displacement: lagging_span_displacement,
        }
    }

    fn turning_line(&self, index: usize) -> Option<f64> {
        // aka Tenkan-sen, or conversion-line

        let lowest_val = match lowest(self.chart.low().offset(index), self.turning_line_period) {
            None => {
                return None;
            }
            Some(lowest_val) => lowest_val,
        };

        let highest_val = match highest(self.chart.high().offset(index), self.turning_line_period) {
            None => {
                return None;
            }
            Some(highest_val) => highest_val,
        };


        let result = (lowest_val + highest_val) / 2.0;

        Some(result)
    }

    fn standard_line(&self, index: usize) -> Option<f64> {
        // aka Kijun-sen or base-line

        let lowest_val = match lowest(self.chart.low().offset(index), self.standard_line_period) {
            None => {
                return None;
            }
            Some(lowest_val) => lowest_val,
        };

        let highest_val = match highest(self.chart.high().offset(index), self.standard_line_period)
        {
            None => {
                return None;
            }
            Some(highest_val) => highest_val,
        };


        let result = (lowest_val + highest_val) / 2.0;

        Some(result)
    }

    fn span_a(&self, index: usize) -> Option<f64> {
        // Senkou Span A (Leading Span A)

        let turning_line_val = match self.turning_line(index) {
            None => {
                return None;
            }
            Some(x) => x,
        };

        let standard_line_val = match self.standard_line(index) {
            None => {
                return None;
            }
            Some(x) => x,
        };

        let result = (turning_line_val + standard_line_val) / 2.0;

        // TODO: account for lagging_span_displacement

        Some(result)
    }

    fn span_b(&self, index: usize) -> Option<f64> {
        // Senkou Span B (Leading Span B)

        let lowest_val = match lowest(self.chart.low().offset(index), self.span_b_period) {
            None => {
                return None;
            }
            Some(lowest_val) => lowest_val,
        };

        let highest_val = match highest(self.chart.high().offset(index), self.span_b_period) {
            None => {
                return None;
            }
            Some(highest_val) => highest_val,
        };


        let result = (lowest_val + highest_val) / 2.0;

        // TODO: account for lagging_span_displacement

        Some(result)
    }

    fn lagging_line(&self, index: usize) -> Option<f64> {
        // TODO: account for lagging_span_displacement

        self.chart.close().get(index)
    }
}
