pub mod prelude {
    pub use super::{
        CpuID,
    };
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
pub struct CpuID(pub u32);

impl std::ops::Deref for CpuID {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CpuID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CpuID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CpuID {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CpuID(u32::from_str(s)?))
    }
}
