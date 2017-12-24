mod sma;
pub use self::sma::SimpleMovingAverage;

mod ema;
pub use self::ema::ExponentialMovingAverage;

mod wma;
pub use self::wma::WeightedMovingAverage;

mod ichimoku;
pub use self::ichimoku::Ichimoku;
