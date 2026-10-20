use crate::prelude::*;

pub mod prelude {
    pub use super::{
        Time2
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Time2 {
    pub value_ns_2: f64,
}

impl Time2 {
    pub fn from_nanos_sqr(value: f64) -> Self {
        Self { value_ns_2: value }
    }

    pub fn as_nanos_sqr(&self) -> f64 {
        self.value_ns_2
    }

    pub fn sqrt(self) -> Time {
        Time::nanos(self.value_ns_2.sqrt())
    }
}

impl std::ops::Neg for Time2 {
    type Output = Time2;

    fn neg(self) -> Self::Output {
        Self::Output { value_ns_2: -self.value_ns_2 }
    }
}

impl std::ops::Add for Time2 {
    type Output = Time2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 + rhs.value_ns_2) }
    }
}

impl std::ops::Sub for Time2 {
    type Output = Time2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 - rhs.value_ns_2) }
    }
}

impl std::ops::Mul<Time> for Time {
    type Output = Time2;

    fn mul(self, rhs: Time) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns * rhs.value_ns) }
    }
}

impl std::ops::Mul<f64> for Time2 {
    type Output = Time2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 * rhs) }
    }
}

impl std::ops::Mul<Time2> for f64 {
    type Output = Time2;

    fn mul(self, rhs: Time2) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Time> for Time2 {
    type Output = Time;

    fn div(self, rhs: Time) -> Self::Output {
        Self::Output { value_ns: self.value_ns_2 / rhs.value_ns }
    }
}

impl std::ops::Div<f64> for Time2 {
    type Output = Time2;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns_2: self.value_ns_2 / rhs }
    }
}
