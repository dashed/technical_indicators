// local imports

use charts::Chart;
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
    pub fn new(
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

    /// Return Ichimoku chart with default settings.
    pub fn default(chart: &'chart Chart) -> Self {
        // reference:
        // http://www.ichimokutrader.com/elements.html
        // http://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:ichimoku_cloud

        Ichimoku {
            chart: chart,

            // default params
            turning_line_period: 9,
            standard_line_period: 26,
            span_b_period: 52,
            lagging_span_displacement: 26,
        }
    }

    /// A moving average of the highest high and lowest low over the last
    /// `turning_line_period` data points.
    pub fn turning_line(&self, index: usize) -> Option<f64> {
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

        // get the average

        let result = (lowest_val + highest_val) / 2.0;

        Some(result)
    }

    /// A moving average of the highest high and lowest low over the last
    /// `standard_line_period` data points.
    pub fn standard_line(&self, index: usize) -> Option<f64> {
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

    /// The average of the turning line (i.e Tenkan Sen) and
    /// standard line (i.e. Kijun Sen), plotted `lagging_span_displacement`
    /// data points ahead.
    pub fn span_a(&self, index: i64) -> Option<f64> {
        // Senkou Span A (Leading Span A)

        // Span A plotted at `index` relies on data that is self.lagging_span_displacement
        // data points behind.

        let normalized_index = index + (self.lagging_span_displacement as i64);

        if normalized_index < 0 {
            return None;
        }

        let normalized_index = normalized_index as usize;

        let turning_line_val = match self.turning_line(normalized_index) {
            None => {
                return None;
            }
            Some(x) => x,
        };

        let standard_line_val = match self.standard_line(normalized_index) {
            None => {
                return None;
            }
            Some(x) => x,
        };

        // get the average

        let result = (turning_line_val + standard_line_val) / 2.0;

        Some(result)
    }

    /// The average of the highest high and lowest low over the last `span_b_period`
    /// data points, plotted `lagging_span_displacement` data points ahead.
    pub fn span_b(&self, index: i64) -> Option<f64> {
        // Senkou Span B (Leading Span B)

        // Span B plotted at `index` relies on data that is self.lagging_span_displacement
        // data points behind.

        let normalized_index = index + (self.lagging_span_displacement as i64);

        if normalized_index < 0 {
            return None;
        }

        let normalized_index = normalized_index as usize;

        let lowest_val = match lowest(
            self.chart.low().offset(normalized_index),
            self.span_b_period,
        ) {
            None => {
                return None;
            }
            Some(lowest_val) => lowest_val,
        };

        let highest_val = match highest(
            self.chart.high().offset(normalized_index),
            self.span_b_period,
        ) {
            None => {
                return None;
            }
            Some(highest_val) => highest_val,
        };

        // get the average

        let result = (lowest_val + highest_val) / 2.0;

        Some(result)
    }

    /// The closing price plotted `lagging_span_displacement` data points behind.
    pub fn lagging_line(&self, index: i64) -> Option<f64> {
        // The ladding line plotted at `index` relies on data that is self.lagging_span_displacement
        // data points ahead.

        let normalized_index = index - (self.lagging_span_displacement as i64);

        if normalized_index < 0 {
            return None;
        }

        let normalized_index = normalized_index as usize;

        self.chart.close().get(normalized_index)
    }
}
