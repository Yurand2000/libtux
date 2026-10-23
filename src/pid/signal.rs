use crate::prelude::*;

pub mod prelude {
    pub use super::{
        Signal,
        signal,
        set_handler,
        set_handler_default,
        set_handler_ignore,
    };
}

const SIG_NUMS: usize = 32;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Signal {
    SIGHUP    = libc::SIGHUP,
    SIGINT    = libc::SIGINT,
    SIGQUIT   = libc::SIGQUIT,
    SIGILL    = libc::SIGILL,
    SIGTRAP   = libc::SIGTRAP,
    SIGABRT   = libc::SIGABRT,
    SIGBUS    = libc::SIGBUS,
    SIGFPE    = libc::SIGFPE,
    SIGKILL   = libc::SIGKILL,
    SIGUSR1   = libc::SIGUSR1,
    SIGSEGV   = libc::SIGSEGV,
    SIGUSR2   = libc::SIGUSR2,
    SIGPIPE   = libc::SIGPIPE,
    SIGALRM   = libc::SIGALRM,
    SIGTERM   = libc::SIGTERM,
    SIGSTKFLT = libc::SIGSTKFLT,
    SIGCHLD   = libc::SIGCHLD,
    SIGCONT   = libc::SIGCONT,
    SIGSTOP   = libc::SIGSTOP,
    SIGTSTP   = libc::SIGTSTP,
    SIGTTIN   = libc::SIGTTIN,
    SIGTTOU   = libc::SIGTTOU,
    SIGURG    = libc::SIGURG,
    SIGXCPU   = libc::SIGXCPU,
    SIGXFSZ   = libc::SIGXFSZ,
    SIGVTALRM = libc::SIGVTALRM,
    SIGPROF   = libc::SIGPROF,
    SIGWINCH  = libc::SIGWINCH,
    SIGIO     = libc::SIGIO,
    SIGPWR    = libc::SIGPWR,
    SIGSYS    = libc::SIGSYS,
}

impl Into<libc::c_int> for Signal {
    fn into(self) -> libc::c_int {
        self as libc::c_int
    }
}

impl From<libc::c_int> for Signal {
    fn from(value: libc::c_int) -> Self {
        use Signal::*;
        match value {
            libc::SIGHUP    => SIGHUP,
            libc::SIGINT    => SIGINT,
            libc::SIGQUIT   => SIGQUIT,
            libc::SIGILL    => SIGILL,
            libc::SIGTRAP   => SIGTRAP,
            libc::SIGABRT   => SIGABRT,
            libc::SIGBUS    => SIGBUS,
            libc::SIGFPE    => SIGFPE,
            libc::SIGKILL   => SIGKILL,
            libc::SIGUSR1   => SIGUSR1,
            libc::SIGSEGV   => SIGSEGV,
            libc::SIGUSR2   => SIGUSR2,
            libc::SIGPIPE   => SIGPIPE,
            libc::SIGALRM   => SIGALRM,
            libc::SIGTERM   => SIGTERM,
            libc::SIGSTKFLT => SIGSTKFLT,
            libc::SIGCHLD   => SIGCHLD,
            libc::SIGCONT   => SIGCONT,
            libc::SIGSTOP   => SIGSTOP,
            libc::SIGTSTP   => SIGTSTP,
            libc::SIGTTIN   => SIGTTIN,
            libc::SIGTTOU   => SIGTTOU,
            libc::SIGURG    => SIGURG,
            libc::SIGXCPU   => SIGXCPU,
            libc::SIGXFSZ   => SIGXFSZ,
            libc::SIGVTALRM => SIGVTALRM,
            libc::SIGPROF   => SIGPROF,
            libc::SIGWINCH  => SIGWINCH,
            libc::SIGIO     => SIGIO,
            libc::SIGPWR    => SIGPWR,
            libc::SIGSYS    => SIGSYS,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum SignalError {
    UnsupportedSignal,
    PermissionError,
    UnknownThread,
    SyscallError(std::io::Error),
}

impl std::fmt::Display for SignalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalError::UnsupportedSignal => write!(f, "Unsupported Signal"),
            SignalError::PermissionError => write!(f, "Permission Error"),
            SignalError::UnknownThread => write!(f, "Unknown Thread"),
            SignalError::SyscallError(error) => write!(f, "Syscall Error: {error}"),
        }
    }
}

impl std::error::Error for SignalError {}

pub fn signal(pid: Pid, sig: Signal) -> Result<(), SignalError> {
    use libc::*;

    unsafe {
        let res = kill(*pid as c_int, sig.into());

        if res != 0 {
            match *__errno_location() {
                EINVAL => Err(SignalError::UnsupportedSignal),
                EPERM => Err(SignalError::PermissionError),
                ESRCH => Err(SignalError::UnknownThread),
                _ => Err(SignalError::SyscallError(std::io::Error::last_os_error())),
            }
        } else {
            Ok(())
        }
    }
}

pub trait SigHandler: FnMut(Signal) + Sync + Send + 'static {}

static SIG_HANDLERS: std::sync::LazyLock<std::sync::Mutex<[Option<Box<dyn SigHandler>>; 32]>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new([const { None }; SIG_NUMS]));

impl<F> SigHandler for F where F: Sync + Send + FnMut(Signal) + 'static {}

pub fn set_handler<F>(sig: Signal, user_handler: F) -> Result<(), SignalError>
where
    F: SigHandler,
{
    {
        let mut guard = SIG_HANDLERS.lock().unwrap();
        let handlers = guard.as_mut();
        handlers[sig as usize] = Some(Box::new(user_handler));
    }
    inner_set_handler(
        sig.into(),
        wrapper_handler as *const () as libc::sighandler_t,
    )
}

pub fn set_handler_ignore(sig: Signal) -> Result<(), SignalError> {
    {
        let mut guard = SIG_HANDLERS.lock().unwrap();
        let handlers = guard.as_mut();
        handlers[sig as usize] = None;
    }

    inner_set_handler(sig.into(), libc::SIG_IGN)
}

pub fn set_handler_default(sig: Signal) -> Result<(), SignalError> {
    {
        let mut guard = SIG_HANDLERS.lock().unwrap();
        let handlers = guard.as_mut();
        handlers[sig as usize] = None;
    }

    inner_set_handler(sig.into(), libc::SIG_DFL)
}

extern "C" fn wrapper_handler(sig: libc::c_int) {
    let mut guard = SIG_HANDLERS.lock().unwrap();
    let vec = guard.as_mut();
    let Some(ref mut fun) = vec[sig as usize] else {
        return;
    };

    fun(Signal::from(sig))
}

fn inner_set_handler(sig: libc::c_int, handler: libc::sighandler_t) -> Result<(), SignalError> {
    unsafe {
        let res = libc::signal(sig, handler);

        if res == libc::SIG_ERR {
            let errno = libc::__errno_location();
            match *errno {
                libc::EINVAL => Err(SignalError::UnsupportedSignal),
                _ => Err(SignalError::SyscallError(std::io::Error::last_os_error())),
            }
        } else {
            Ok(())
        }
    }
}
