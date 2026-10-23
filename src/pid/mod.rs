pub mod prelude {
    #[cfg(feature = "sched")]
    pub use super::sched::prelude::*;

    #[cfg(feature = "cpuset")]
    pub use super::cpuset::prelude::*;

    #[cfg(feature = "signal")]
    pub use super::signal::prelude::*;

    pub use super::{
        Pid,
    };
}

#[cfg(feature = "sched")]
pub mod sched;

#[cfg(feature = "cpuset")]
pub mod cpuset;

#[cfg(feature = "signal")]
pub mod signal;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
pub struct Pid(pub u32);

impl std::ops::Deref for Pid {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Pid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Pid {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pid(u32::from_str(s)?))
    }
}
