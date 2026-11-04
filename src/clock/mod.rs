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

    timespec.into()
}

pub fn clock_sleep_relative(clock: ClockType, time: Time) {
    let timespec = time.into();

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
    let timespec = time.into();

    unsafe {
        libc::clock_nanosleep(
            clock.into(),
            libc::TIMER_ABSTIME,
            &timespec,
            std::ptr::null_mut()
        );
    };
}
