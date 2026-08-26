use crate::prelude::*;

pub mod prelude {
    #[cfg(feature = "serde")]
    pub use super::time_serde::TimeParseError;
}

impl Time {
    pub const MICRO_TO_NANO: f64 = 1000.0;
    pub const MILLI_TO_NANO: f64 = 1000_000.0;
    pub const  SECS_TO_NANO: f64 = 1000_000_000.0;

    pub fn zero() -> Self {
        Self { value_ns: 0.0 }
    }

    pub fn one() -> Self {
        Self { value_ns: 1.0 }
    }

    pub fn infinity() -> Self {
        Self { value_ns: f64::INFINITY }
    }

    pub fn nanos(time_ns: f64) -> Self {
        Self { value_ns: time_ns }
    }

    pub fn micros(time_us: f64) -> Self {
        Self { value_ns: time_us * Self::MICRO_TO_NANO }
    }

    pub fn millis(time_ms: f64) -> Self {
        Self { value_ns: time_ms * Self::MILLI_TO_NANO }
    }

    pub fn secs(time_s: f64) -> Self {
        Self { value_ns: time_s * Self::SECS_TO_NANO }
    }

    pub fn as_nanos(&self) -> f64 {
        self.value_ns
    }

    pub fn as_micros(&self) -> f64 {
        self.value_ns / Self::MICRO_TO_NANO
    }

    pub fn as_millis(&self) -> f64 {
        self.value_ns / Self::MILLI_TO_NANO
    }

    pub fn as_secs(&self) -> f64 {
        self.value_ns / Self::SECS_TO_NANO
    }

    pub fn floor(self) -> Self {
        Self { value_ns: f64::floor(self.value_ns) }
    }

    pub fn ceil(self) -> Self {
        Self { value_ns: f64::ceil(self.value_ns) }
    }

    pub fn round(self) -> Self {
        Self { value_ns: f64::round(self.value_ns) }
    }
}

// Comparisons
impl Time {
    fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Since the unit is in nanoseconds, two times are equal if they differ by half of that.
        const ERROR: f64 = 0.5;

        if f64::abs(self.value_ns - other.value_ns) < ERROR {
            std::cmp::Ordering::Equal
        } else {
            f64::total_cmp(&self.value_ns, &other.value_ns)
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        Self::total_cmp(&self, other).is_eq()
    }
}

impl Eq for Time { }

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Self::total_cmp(&self, other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Self::total_cmp(&self, other)
    }
}

// Operations
impl std::ops::Neg for Time {
    type Output = Time;

    fn neg(self) -> Self::Output {
        Self::Output { value_ns: -self.value_ns }
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: (self.value_ns + rhs.value_ns) }
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: (self.value_ns - rhs.value_ns) }
    }
}

impl std::ops::Mul<f64> for Time {
    type Output = Time;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns: (self.value_ns * rhs) }
    }
}

impl std::ops::Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div for Time {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.value_ns / rhs.value_ns
    }
}

impl std::ops::Div<f64> for Time {
    type Output = Time;

    fn div(self, rhs: f64) -> Self::Output {
        Time { value_ns: self.value_ns / rhs }
    }
}

impl std::ops::Rem for Time {
    type Output = Time;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: self.value_ns.floor() % rhs.value_ns.floor() }
    }
}

impl std::iter::Sum for Time {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Time::zero(), |acc, val| acc + val)
    }
}

// Display
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.value_ns.floor() as i64;
        if nanos % Time::SECS_TO_NANO as i64 == 0 {
            write!(f, "{}s", nanos / Time::SECS_TO_NANO as i64)
        } else if nanos % Time::MILLI_TO_NANO as i64 == 0 {
            write!(f, "{}ms", nanos / Time::MILLI_TO_NANO as i64)
        } else if nanos % Time::MICRO_TO_NANO as i64 == 0 {
            write!(f, "{}us", nanos / Time::MICRO_TO_NANO as i64)
        } else {
            write!(f, "{}ns", nanos)
        }
    }
}

// Serialization
#[cfg(feature = "serde")]
mod time_serde {
    use crate::prelude::*;

    #[derive(Debug)]
    pub enum TimeParseError {
        NumberParseError(std::num::ParseFloatError),
        UnitParseError(String),
        FormatError(),
    }

    impl std::fmt::Display for TimeParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use TimeParseError::*;

            match self {
                NumberParseError(err) => write!(f, "Invalid Number: {err}"),
                UnitParseError(unit) => write!(f, "Invalid Time Unit: {unit}"),
                FormatError() => write!(f, "Invalid Format"),
            }
        }
    }

    impl std::error::Error for TimeParseError { }

    impl std::str::FromStr for Time {
        type Err = TimeParseError;

        fn from_str(str: &str) -> Result<Self, Self::Err> {
            let Some(split_idx) =
                str.char_indices().find_map(|(index, ch)| {
                    if !(ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '.' || ch == 'e' || ch == 'E') {
                        Some(index)
                    } else {
                        None
                    }
                }) else {
                    return Err(TimeParseError::FormatError())
                };

            let (num, unit) = str.split_at(split_idx);

            let unit = match unit {
                "s" => Time::SECS_TO_NANO,
                "ms" => Time::MILLI_TO_NANO,
                "us" => Time::MICRO_TO_NANO,
                "ns" => 1.0,
                u => { return Err(TimeParseError::UnitParseError(u.to_owned())) }
            };

            let num: f64 = num.parse()
                .map_err(|err| TimeParseError::NumberParseError(err))?;

            Ok(Time::nanos(num * unit))
        }
    }

    impl serde::Serialize for Time {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            format!("{}", self).serialize(serializer)
        }
    }

    impl<'de> serde::Deserialize<'de> for Time {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(serde::de::Error::custom)
        }
    }

}

