use crate::prelude::*;

pub mod prelude {
    pub use super::{
        CpuSet,
        get_cpuset_to_pid,
        set_cpuset_to_pid,
    };
}

/// Set of valid CPUs on the current machine
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct CpuSet {
    cpus: Vec<CpuID>,
}

#[derive(Debug)]
pub enum CpuSetBuildError {
    IO(std::io::Error),
    ParseError(String),
    UnavailableCPUs,
}

impl std::fmt::Display for CpuSetBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuSetBuildError::IO(error) => write!(f, "IO error: {error}"),
            CpuSetBuildError::ParseError(error) => write!(f, "Parse error: {error}"),
            CpuSetBuildError::UnavailableCPUs => write!(f, "Requesting more CPUs than available ones"),
        }
    }
}

impl std::error::Error for CpuSetBuildError {}

#[derive(Debug)]
pub enum GetAffinityError {
    UnknownThread,
    SyscallError(std::io::Error),
}

#[derive(Debug)]
pub enum SetAffinityError {
    UnknownThread,
    InvalidMask,
    PermissionError,
    SyscallError(std::io::Error),
}

impl std::fmt::Display for GetAffinityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAffinityError::UnknownThread => write!(f, "Unknown Thread"),
            GetAffinityError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
        }
    }
}

impl std::error::Error for GetAffinityError {}

impl std::fmt::Display for SetAffinityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetAffinityError::UnknownThread => write!(f, "Unknown Thread"),
            SetAffinityError::InvalidMask => write!(f, "Invalid cpu mask"),
            SetAffinityError::PermissionError => write!(f, "Permission Error"),
            SetAffinityError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
        }
    }
}

impl std::error::Error for SetAffinityError {}

impl CpuSet {
    /// Create a CpuSet with only the given CPU
    pub fn single(cpu: CpuID) -> Self  {
        CpuSet { cpus: vec![cpu] }
    }

    /// Create an empty CpuSet
    pub fn empty() -> Self {
        CpuSet { cpus: Vec::with_capacity(0) }
    }

    /// Create a CpuSet containing all online cpus
    #[cfg(feature = "parser")]
    pub fn online() -> Result<Self, CpuSetBuildError> {
        use std::str::FromStr as _;

        let online_cpus = std::fs::read_to_string("/sys/devices/system/cpu/online")
            .map_err(|err| CpuSetBuildError::IO(err))?;
        let cpuset = CpuSet::from_str(&online_cpus)?;

        Ok(CpuSet { cpus: cpuset.cpus })
    }

    /// Create a CpuSet subset containing a given number of CPUs
    pub fn any_subset(self, num_cpus: u64) -> Result<Self, CpuSetBuildError> {
        if num_cpus as usize > self.cpus.len() {
            return Err(CpuSetBuildError::UnavailableCPUs);
        }

        Ok(CpuSet {
            cpus: self.cpus.into_iter().take(num_cpus as usize).collect()
        })
    }

    /// Add a CPU to the set
    pub fn add_cpu(mut self, cpu: CpuID) -> Self {
        if !self.cpus.contains(&cpu) {
            self.cpus.push(cpu);
        }

        self
    }

    /// Remove a CPU from the set
    pub fn remove_cpu(mut self, cpu: CpuID) -> Self {
        match self.cpus.iter().position(|elem| elem == &cpu) {
            Some(i) => { self.cpus.swap_remove(i); },
            None => (),
        };

        self
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.cpus.iter()
            .all(|cpu| other.cpus.contains(cpu))
    }

    /// Get if a CPU is in the set
    pub fn has_cpu(&self, cpu: CpuID) -> bool {
        self.cpus.iter().any(|&my_cpu| my_cpu == cpu)
    }

    /// Return the number of CPUs in the set
    pub fn num_cpus(&self) -> usize {
        self.cpus.len()
    }

    /// Returns an iterator over the CPU set
    pub fn iter(&self) -> impl Iterator<Item = &CpuID> {
        self.cpus.iter()
    }

    /// Returns a mutable iterator over the CPU set
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CpuID> {
        self.cpus.iter_mut()
    }

    /// Creates a consuming iterator over the CPU set
    pub fn into_iter(self) -> impl Iterator<Item = CpuID> {
        self.cpus.into_iter()
    }

    /// Return the number of configured CPUs in the system
    pub fn system_cpus() -> usize {
        unsafe { libc::sysconf(libc::_SC_NPROCESSORS_CONF) as usize }
    }

    /// Return the number of online CPUs in the system
    pub fn online_cpus() -> usize {
        unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as usize }
    }
}

#[cfg(feature = "parser")]
impl std::str::FromStr for CpuSet {
    type Err = CpuSetBuildError;

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        use nom::Parser;
        use nom::bytes::complete::*;
        use nom::branch::*;
        use nom::multi::*;
        use nom::character::complete::*;
        use nom::combinator::*;

        let single_parser = || map_res(digit1::<&str, ()>, |s: &str| s.parse::<CpuID>());
        let single_parser_pair = map(single_parser(), |cpu| (cpu, cpu) );
        let range_parser = map_res(
            (
                single_parser(),
                tag("-"),
                single_parser()
            ),
            |(min, _, max)| {
                if min > max {
                    Err(format!("Range error"))
                } else {
                    Ok((min, max))
                }
            }
        );

        let separator_parser = map((tag(","), multispace0), |_| ());
        let mut parser = map(
            separated_list1(
                separator_parser,
                alt((range_parser, single_parser_pair))
            ),
            |pairs: Vec<(CpuID, CpuID)>| {
                let mut out: Vec<CpuID> = Vec::new();
                for pair in pairs.into_iter() {
                    for cpu in *pair.0 ..= *pair.1 {
                        out.push(CpuID(cpu));
                    }
                }

                out
            }
        );

        Ok(CpuSet {
            cpus: parser.parse(s)
                .map_err(|err| CpuSetBuildError::ParseError(format!("{err}")))?.1
        })
    }
}

impl std::fmt::Display for CpuSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.cpus.iter().peekable();
        if iter.peek().is_some() {
            let cpu = iter.next().unwrap();
            write!(f, "{cpu}")?;

            for cpu in iter {
                write!(f, ", {cpu}")?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl TryInto<libc::cpu_set_t> for &CpuSet {
    type Error = SetAffinityError;

    fn try_into(self) -> Result<libc::cpu_set_t, Self::Error> {
        let mut cpuset: libc::cpu_set_t;

        unsafe {
            cpuset = std::mem::zeroed();

            let num_cpus = libc::CPU_SETSIZE as u32;

            for cpu in self.iter() {
                if **cpu >= num_cpus {
                    return Err(SetAffinityError::InvalidMask);
                }

                libc::CPU_SET(**cpu as usize, &mut cpuset);
            }
        }

        Ok(cpuset)
    }
}

impl From<libc::cpu_set_t> for CpuSet {
    fn from(value: libc::cpu_set_t) -> Self {
        let mut cpuset = Self::empty();

        unsafe {
            let num_cpus = libc::CPU_SETSIZE as usize;

            for i in 0..num_cpus {
                if libc::CPU_ISSET(i, &value) {
                    cpuset = cpuset.add_cpu(CpuID(i as u32));
                }
            }
        }

        cpuset
    }
}

/// Get affinity to given PID
pub fn get_cpuset_to_pid(pid: Pid) -> Result<CpuSet, GetAffinityError> {
    use libc::*;

    unsafe {
        let mut cpuset: cpu_set_t = std::mem::zeroed();

        let res =
            sched_getaffinity(
                *pid                    as pid_t,
                size_of::<cpu_set_t>()  as size_t,
                &mut cpuset             as *mut cpu_set_t,
            );

        if res != 0 {
            match *__errno_location() {
                ESRCH => Err(GetAffinityError::UnknownThread),
                _ => Err(GetAffinityError::SyscallError(std::io::Error::last_os_error())),
            }
        } else {
            Ok(cpuset.into())
        }
    }
}

/// Set affinity to given PID
pub fn set_cpuset_to_pid(pid: Pid, cpu_set: &CpuSet) -> Result<(), SetAffinityError> {
    use libc::*;

    let cpuset: cpu_set_t = cpu_set.try_into()?;

    unsafe {
        let res =
            sched_setaffinity(
                *pid                    as pid_t,
                size_of::<cpu_set_t>()  as size_t,
                &cpuset                 as *const cpu_set_t,
            );

        if res != 0 {
            match *__errno_location() {
                EINVAL => Err(SetAffinityError::InvalidMask),
                ESRCH => Err(SetAffinityError::UnknownThread),
                EPERM => Err(SetAffinityError::PermissionError),
                _ => Err(SetAffinityError::SyscallError(std::io::Error::last_os_error())),
            }
        } else {
            Ok(())
        }
    }
}
