use crate::prelude::*;

pub mod prelude {
    pub use super::{
        SchedPolicy,
        SchedFlags,
        get_sched_policy,
        set_sched_policy,
    };
}

/// Scheduling Policy
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum SchedPolicy {
    OTHER { nice: i32 },
    BATCH { nice: i32 },
    IDLE,
    FIFO(i32),
    RR(i32),
    DEADLINE {
        runtime_ms: u64,
        deadline_ms: u64,
        period_ms: u64,
    }
}

impl SchedPolicy {
    /// Builder for SCHED_OTHER at niceness zero
    pub fn other() -> Self { SchedPolicy::OTHER { nice: 0 }}

    pub fn is_other(&self) -> bool {
        match self {
            Self::OTHER { .. } => true,
            _ => false,
        }
    }

    pub fn is_fifo_rr(&self) -> bool {
        match self {
            Self::FIFO(_) => true,
            Self::RR(_) => true,
            _ => false,
        }
    }

    pub fn is_deadline(&self) -> bool {
        match self {
            Self::DEADLINE { .. } => true,
            _ => false,
        }
    }
}

/// Scheduling Flags
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub struct SchedFlags(u32);

bitflags::bitflags! {
    impl SchedFlags: u32 {
        const RESET_ON_FORK = 1;
        const RECLAIM = 2;
    }
}

#[derive(Debug)]
pub struct GetSchedError {
    pid: Pid,
    error: GetSchedPolicyError,
}

#[derive(Debug)]
pub struct SetSchedError {
    pid: Pid,
    error: SetSchedPolicyError,
}

impl std::error::Error for GetSchedError {}

impl std::fmt::Display for GetSchedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error for PID {}: {}", self.pid, self.error)
    }
}

impl std::error::Error for SetSchedError {}

impl std::fmt::Display for SetSchedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error for PID {}: {}", self.pid, self.error)
    }
}

#[derive(Debug)]
pub enum GetSchedPolicyError {
    SyscallError(std::io::Error),
    UnknownPolicy(i32),
}

#[derive(Debug)]
pub enum SetSchedPolicyError {
    SyscallError(std::io::Error),
    DeadlineWOResetOnFork,
}

impl std::fmt::Display for GetSchedPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GetSchedPolicyError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
            GetSchedPolicyError::UnknownPolicy(policy)=> write!(f, "Unknown Policy: {policy}"),
        }
    }
}

impl std::fmt::Display for SetSchedPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SetSchedPolicyError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
            SetSchedPolicyError::DeadlineWOResetOnFork => write!(f, "Setting Deadline policy requires the RESET_ON_FORK flag"),
        }
    }
}

struct SchedData {
    policy: SchedPolicy,
    flags: SchedFlags,
}

impl TryFrom<libc::sched_attr> for SchedData {
    type Error = GetSchedPolicyError;

    fn try_from(value: libc::sched_attr) -> Result<Self, Self::Error> {
        let policy = match value.sched_policy as i32 {
            libc::SCHED_OTHER => SchedPolicy::OTHER { nice: value.sched_nice },
            libc::SCHED_BATCH => SchedPolicy::BATCH { nice: value.sched_nice },
            libc::SCHED_IDLE => SchedPolicy::IDLE,
            libc::SCHED_FIFO => SchedPolicy::FIFO( value.sched_priority as i32 ),
            libc::SCHED_RR => SchedPolicy::RR( value.sched_priority as i32 ),
            libc::SCHED_DEADLINE => SchedPolicy::DEADLINE {
                runtime_ms: value.sched_runtime,
                deadline_ms: value.sched_deadline,
                period_ms: value.sched_period,
            },
            val => { return Err(GetSchedPolicyError::UnknownPolicy(val)); }
        };

        let mut flags = SchedFlags::empty();
        if (value.sched_flags & libc::SCHED_FLAG_RESET_ON_FORK as u64) > 0 {
            flags |= SchedFlags::RESET_ON_FORK;
        }
        if (value.sched_flags & libc::SCHED_FLAG_RECLAIM as u64) > 0 {
            flags |= SchedFlags::RECLAIM;
        }

        Ok(Self { policy, flags })
    }
}

impl TryInto<libc::sched_attr> for SchedData {
    type Error = SetSchedPolicyError;

    fn try_into(self) -> Result<libc::sched_attr, Self::Error> {
        let SchedData { policy, flags } = self;

        let sched_policy = match policy {
            SchedPolicy::OTHER { .. } => libc::SCHED_OTHER,
            SchedPolicy::BATCH { .. } => libc::SCHED_BATCH,
            SchedPolicy::IDLE => libc::SCHED_IDLE,
            SchedPolicy::FIFO(_) => libc::SCHED_FIFO,
            SchedPolicy::RR(_) => libc::SCHED_RR,
            SchedPolicy::DEADLINE { .. } => libc::SCHED_DEADLINE,
        } as u32;

        let mut sched_flags = 0u64;
        if let SchedPolicy::DEADLINE { .. } = policy {
            if !flags.contains(SchedFlags::RESET_ON_FORK) {
                return Err(SetSchedPolicyError::DeadlineWOResetOnFork);
            }
        }

        if flags.contains(SchedFlags::RESET_ON_FORK) {
            sched_flags |= libc::SCHED_FLAG_RESET_ON_FORK as u64;
        }

        if flags.contains(SchedFlags::RECLAIM) {
            sched_flags |= libc::SCHED_FLAG_RECLAIM as u64;
        }

        let sched_nice = match policy {
            SchedPolicy::OTHER { nice } => nice,
            SchedPolicy::BATCH { nice } => nice,
            _ => 0,
        };

        let sched_priority = match policy {
            SchedPolicy::FIFO(prio) => prio,
            SchedPolicy::RR(prio) => prio,
            _ => 0,
        } as u32;

        const MILLI_TO_NANO: u64 = 1000_000;
        let (sched_runtime, sched_deadline, sched_period) =
            match policy {
                SchedPolicy::DEADLINE { runtime_ms, deadline_ms, period_ms }
                    => (
                        runtime_ms * MILLI_TO_NANO,
                        deadline_ms * MILLI_TO_NANO,
                        period_ms * MILLI_TO_NANO
                    ),
                _ => (0, 0, 0),
            };

        Ok(libc::sched_attr {
            size: size_of::<libc::sched_attr>() as u32,
            sched_policy,
            sched_flags,
            sched_nice,
            sched_priority,
            sched_runtime,
            sched_deadline,
            sched_period,
        })
    }
}

/// Get scheduling policy of the given PID
pub fn get_sched_policy(pid: Pid) -> Result<(SchedPolicy, SchedFlags), GetSchedError> {
    use libc::*;

    unsafe {
        let mut attr = libc::sched_attr {
            size: 0,
            sched_policy: 0,
            sched_flags: 0,
            sched_nice: 0,
            sched_priority: 0,
            sched_runtime: 0,
            sched_deadline: 0,
            sched_period: 0,
        };

        let res =
            syscall(
                SYS_sched_getattr,
                pid.0                           as pid_t,
                &mut attr                       as *mut sched_attr,
                size_of::<sched_attr>()   as c_uint,
                0                               as c_uint,
            );

        if res != 0 {
            Err(GetSchedError { pid, error: GetSchedPolicyError::SyscallError(std::io::Error::last_os_error())})
        } else {
            attr.try_into()
                .map(|data: SchedData| (data.policy, data.flags))
                .map_err(|error| GetSchedError { pid, error })
        }
    }
}

/// Set the scheduling policy of the given PID
pub fn set_sched_policy(pid: Pid, policy: SchedPolicy, flags: SchedFlags) -> Result<(), SetSchedError> {
    use libc::*;

    let res;

    unsafe {
        let attr: sched_attr = SchedData{ policy, flags }.try_into()
            .map_err(|error| SetSchedError { pid, error })?;

        res =
            syscall(
                SYS_sched_setattr,
                pid.0                       as pid_t,
                &attr                       as *const sched_attr,
                0                           as c_uint,
            );
    };

    if res != 0 {
        Err(SetSchedError { pid, error: SetSchedPolicyError::SyscallError(std::io::Error::last_os_error())})
    } else {
        Ok(())
    }
}
