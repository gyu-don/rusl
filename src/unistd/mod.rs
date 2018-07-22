mod types;

pub use self::types::*;

use errno_const::*;

pub unsafe fn access(filename: *const i8, amode: i32) -> i32 {
    syscall!(ACCESS, filename, amode) as i32
}

pub unsafe fn acct(filename: *const i8) -> i32 {
    syscall!(ACCT, filename) as i32
}

pub unsafe fn alarm(seconds: u32) -> u32 {
    use super::sys::time::{ITIMER_REAL, itimerval, time_t, timeval};
    let mut it = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: seconds as time_t,
            tv_usec: 0,
        },
    };
    let ptr = &mut it as *mut itimerval;
    syscall!(SETITIMER, ITIMER_REAL, ptr, ptr);
    (it.it_value.tv_sec + if it.it_value.tv_usec != 0 { 1 } else { 0 }) as u32
}

pub unsafe fn chdir(path: *const i8) -> i32 {
    syscall!(CHDIR, path) as i32
}

pub unsafe fn chown(path: *const i8, uid: uid_t, gid: gid_t) -> i32 {
    syscall!(CHOWN, path, uid, gid) as i32
}

pub unsafe fn close(fd: i32) -> i32 {
    let mut r = syscall!(CLOSE, fd) as i32;
    if r == -EINTR {
        r = 0;
    }
    r
}

pub unsafe fn ctermid(s: *mut i8) -> *const i8 {
    if s.is_null() {
        b"/dev/tty\0" as *const u8 as *const i8
    } else {
        (b"/dev/tty\0" as *const u8 as *const i8).copy_to(s, 9);
        s
    }
}

pub unsafe fn dup(fd: i32) -> i32 {
    syscall!(DUP, fd) as i32
}

pub unsafe fn dup2(old: i32, new: i32) -> i32 {
    loop {
        let r = syscall!(DUP2, old, new) as i32;
        if r != -EBUSY {
            return r;
        }
    }
}

pub unsafe fn dup3(old: i32, new: i32, flags: i32) -> i32 {
    loop {
        let r = syscall!(DUP3, old, new, flags) as i32;
        if r != -EBUSY {
            return r;
        }
    }
}

pub fn faccessat(fd: i32, filename: *const i8, amode: i32, flag: i32) -> i32 {
    // TODO: implement
    0
}

pub unsafe fn getpid() -> pid_t {
    syscall!(GETPID) as pid_t
}

pub unsafe fn getppid() -> pid_t {
    syscall!(GETPPID) as pid_t
}
