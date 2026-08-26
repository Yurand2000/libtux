use crate::prelude::*;

pub mod prelude {
    pub use super::{
        MonotonicClock as Monotonic,
        SystemClock as System,
    };
}

pub type MonotonicClock = Clock<Monotonic>;
pub type SystemClock = Clock<System>;

pub struct Monotonic;
pub struct System;

#[derive(Debug, Clone, Copy)]
pub struct Clock<T>(std::marker::PhantomData<T>);

impl Clock<Monotonic> {
    pub fn now() -> Time {
        clock_gettime(ClockType::Monotonic)
    }

    pub fn sleep_relative(time: Time)  {
        clock_sleep_relative(ClockType::Monotonic, time)
    }

    pub fn sleep_absolute(time: Time) {
        clock_sleep_absolute(ClockType::Monotonic, time)
    }
}

impl Clock<System> {
    pub fn now() -> Time {
        clock_gettime(ClockType::Realtime)
    }

    pub fn sleep_relative(time: Time)  {
        clock_sleep_relative(ClockType::Realtime, time)
    }

    pub fn sleep_absolute(time: Time) {
        clock_sleep_absolute(ClockType::Realtime, time)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ClockType {
    Monotonic,
    Realtime,
}

impl Into<libc::clockid_t> for ClockType {
    fn into(self) -> libc::clockid_t {
        match self {
            ClockType::Monotonic => libc::CLOCK_MONOTONIC,
            ClockType::Realtime => libc::CLOCK_REALTIME,
        }
    }
}

pub fn clock_gettime(clock: ClockType) -> Time {
    let mut timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        libc::clock_gettime(
            clock.into(),
            &mut timespec
        );
    }

    timespec_to_time(timespec)
}

pub fn clock_sleep_relative(clock: ClockType, time: Time) {
    let timespec = time_to_timespec(time);

    unsafe {
        libc::clock_nanosleep(
            clock.into(),
            0,
            &timespec,
            std::ptr::null_mut()
        );
    };
}

pub fn clock_sleep_absolute(clock: ClockType, time: Time) {
    let timespec = time_to_timespec(time);

    unsafe {
        libc::clock_nanosleep(
            clock.into(),
            libc::TIMER_ABSTIME,
            &timespec,
            std::ptr::null_mut()
        );
    };
}

pub fn timespec_to_time(timespec: libc::timespec) -> Time {
    Time::secs(timespec.tv_sec as f64) +
        Time::nanos(timespec.tv_nsec as f64)
}

pub fn time_to_timespec(time: Time) -> libc::timespec {
    libc::timespec {
        tv_sec: (time.as_secs().floor()) as i64,
        tv_nsec: (time.as_secs().fract() * Time::SECS_TO_NANO) as i64,
    }
}
