pub mod prelude {
    pub use super::common::prelude::*;

    #[cfg(feature= "clock")]
    pub use super::clock::prelude as clock;

    #[cfg(feature= "cpuset")]
    pub use super::cpu::prelude as cpu;

    #[cfg(any(feature= "sched", feature= "cpuset", feature= "signal"))]
    pub use super::pid::prelude as pid;

    #[cfg(feature= "time")]
    pub use super::time::prelude as time;
}

pub mod common;

#[cfg(feature= "clock")]
pub mod clock;

#[cfg(feature= "cpuset")]
pub mod cpu;

#[cfg(any(feature= "sched", feature= "cpuset", feature= "signal"))]
pub mod pid;

#[cfg(feature= "time")]
pub mod time;
