use crate::prelude::*;

pub mod prelude {
    pub use super::time::prelude::*;
    #[cfg(feature = "time_extra")]
    pub use super::time_extra::prelude::*;
    pub use super::{
        Time,
    };
}

pub mod time;

#[cfg(feature = "time_extra")]
pub mod time_extra;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    value_ns: f64,
}
