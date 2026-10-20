pub mod prelude {
    pub use super::common::prelude::*;

    #[cfg(feature = "clock")]
    pub use super::clock::prelude as clock;

    #[cfg(feature = "cpuset")]
    pub use super::cpu::prelude as cpu;

    #[cfg(any(feature = "sched", feature= "cpuset"))]
    pub use super::pid::prelude as pid;

    #[cfg(feature = "time")]
    pub use super::time::prelude as time;
}

pub mod common;

#[cfg(feature = "clock")]
pub mod clock;

#[cfg(feature = "cpuset")]
pub mod cpu;

#[cfg(any(feature = "sched", feature= "cpuset"))]
pub mod pid;

#[cfg(feature = "time")]
pub mod time;
