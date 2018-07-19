use unistd::types::pid_t;

pub fn getpid() -> pid_t {
    unsafe { syscall!(GETPID) as pid_t }
}
