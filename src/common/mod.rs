use crate::prelude::*;

pub mod prelude {
    pub use super::time::prelude::*;
    #[cfg(feature = "time_extra")]
    pub use super::time_extra::prelude::*;
    #[cfg(feature = "time_iterators")]
    pub use super::time_iterators::prelude::*;

    pub use super::{
        Time,
    };
}

pub mod time;

#[cfg(feature = "time_extra")]
pub mod time_extra;

#[cfg(feature = "time_iterators")]
pub mod time_iterators;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    value_ns: f64,
}
