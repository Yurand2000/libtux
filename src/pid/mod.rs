pub mod prelude {
    pub use super::{
        Pid,
    };
}

#[cfg(feature = "sched")]
pub mod sched;

#[cfg(feature = "cpuset")]
pub mod cpuset;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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
