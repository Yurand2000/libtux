use crate::prelude::*;

pub mod prelude {
    pub use super::{
        TimeIterators,
    };
}

pub struct TimeIterators;

impl TimeIterators {
    pub fn range(start: Time, end: Time) -> impl DoubleEndedIterator<Item = Time> {
        (start.value_ns as usize ..= end.value_ns as usize)
            .map(|time_ns| Time { value_ns: time_ns as f64 })
    }

    pub fn range_w_step(start: Time, end: Time, step: Time) -> impl DoubleEndedIterator<Item = Time> {
        (start.value_ns as usize .. end.value_ns as usize + 1)
            .step_by(step.value_ns as usize)
            .map(|time_ns| Time { value_ns: time_ns as f64 })
    }
}
