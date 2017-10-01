// local imports

use charts::SourceSeries;

/// Utility functions.

/// Get lowest value among `length` data points in given `source`.
pub fn lowest(source: SourceSeries, length: usize) -> Option<f64> {
    if length <= 0 {
        return None;
    }

    let lowest = source.get(0);

    if lowest.is_none() {
        return None;
    }

    let mut lowest = source.get(0).unwrap();

    for index in 1..(length) {
        match source.get(index) {
            None => {}
            Some(maybe_lowest) => if maybe_lowest < lowest {
                lowest = maybe_lowest;
            },
        }
    }

    Some(lowest)
}

/// Get highest value among `length` data points in given `source`.
pub fn highest(source: SourceSeries, length: usize) -> Option<f64> {
    if length <= 0 {
        return None;
    }

    let highest = source.get(0);

    if highest.is_none() {
        return None;
    }

    let mut highest = highest.unwrap();

    for index in 1..(length) {
        match source.get(index) {
            None => {}
            Some(maybe_highest) => if maybe_highest > highest {
                highest = maybe_highest;
            },
        }
    }

    Some(highest)
}
