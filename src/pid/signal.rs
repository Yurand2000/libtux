use crate::prelude::*;

pub mod prelude {
    pub use super::{
        Signal,
        signal,
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Signal {
    SIGHUP      = libc::SIGHUP,
    SIGINT      = libc::SIGINT,
    SIGQUIT     = libc::SIGQUIT,
    SIGILL      = libc::SIGILL,
    SIGTRAP     = libc::SIGTRAP,
    SIGABRT     = libc::SIGABRT,
    SIGBUS      = libc::SIGBUS,
    SIGFPE      = libc::SIGFPE,
    SIGKILL     = libc::SIGKILL,
    SIGUSR1     = libc::SIGUSR1,
    SIGSEGV     = libc::SIGSEGV,
    SIGUSR2     = libc::SIGUSR2,
    SIGPIPE     = libc::SIGPIPE,
    SIGALRM     = libc::SIGALRM,
    SIGTERM     = libc::SIGTERM,
    SIGSTKFLT   = libc::SIGSTKFLT,
    SIGCHLD     = libc::SIGCHLD,
    SIGCONT     = libc::SIGCONT,
    SIGSTOP     = libc::SIGSTOP,
    SIGTSTP     = libc::SIGTSTP,
    SIGTTIN     = libc::SIGTTIN,
    SIGTTOU     = libc::SIGTTOU,
    SIGURG      = libc::SIGURG,
    SIGXCPU     = libc::SIGXCPU,
    SIGXFSZ     = libc::SIGXFSZ,
    SIGVTALRM   = libc::SIGVTALRM,
    SIGPROF     = libc::SIGPROF,
    SIGWINCH    = libc::SIGWINCH,
    SIGIO       = libc::SIGIO,
    SIGPWR      = libc::SIGPWR,
    SIGSYS      = libc::SIGSYS,
}

#[derive(Debug)]
pub enum SignalSendError {
    UnsupportedSignal,
    PermissionError,
    UnknownThread,
    SyscallError(std::io::Error),
}

impl std::fmt::Display for SignalSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalSendError::UnsupportedSignal => write!(f, "Unsupported Signal"),
            SignalSendError::PermissionError => write!(f, "Permission Error"),
            SignalSendError::UnknownThread => write!(f, "Unknown Thread"),
            SignalSendError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
        }
    }
}

impl std::error::Error for SignalSendError {}

pub fn signal(pid: Pid, sig: Signal) -> Result<(), SignalSendError> {
    use libc::*;

    unsafe {
        let res =
            kill(*pid as c_int, sig as c_int);

        if res != 0 {
            match *__errno_location() {
                EINVAL => Err(SignalSendError::UnsupportedSignal),
                EPERM => Err(SignalSendError::PermissionError),
                ESRCH => Err(SignalSendError::UnknownThread),
                _ => Err(SignalSendError::SyscallError(std::io::Error::last_os_error())),
            }
        } else {
            Ok(())
        }
    }
}
