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
        (b"/dev/tty\0" as *const u8 as *const i8).copy_to_nonoverlapping(s, 9);
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

/*
pub fn faccessat(fd: i32, filename: *const i8, amode: i32, flag: i32) -> i32 {
    If flag == 0 || (flag == AT_EACCESS && getuid() == geteuid() && getgid() == getegid()) {
        return syscall!(FACCESSAT, fd, filename, amode, flag) as i32;
    }
    if flag != AT_EACCESS {
        return -EINVAL;
    }
    let mut stack: [u8; 1024];

    // TODO: Implement.
}
*/

// For internal
unsafe fn procfdname(buf: *mut i8, fd: u32) {
    let path = b"/proc/self/fd/";
    buf.copy_from_nonoverlapping(path.as_ptr() as *const i8, path.len());
    if fd == 0 {
        buf.add(path.len()).copy_from_nonoverlapping(b"0\0".as_ptr() as *const i8, 2);
        return;
    }
    let m = {
        let mut fd = fd;
        let mut m = 0;
        while fd > 0 {
            m += 1;
            fd /= 10;
        }
        m
    };
    let mut ptr = buf.add(path.len() + m + 1);
    ptr.write(0);
    ptr = ptr.sub(1);
    for _ in 0..m {
        ptr.write((fd % 10) as i8);
        ptr.sub(1);
    }
}

pub unsafe fn fchdir(fd: i32) -> i32 {
    use fcntl::F_GETFD;
    let r = syscall!(FCHDIR, fd) as i32;
    if r != -EBADF || (syscall!(FCNTL, fd, F_GETFD) as i32) < 0 {
        return r;
    }
    let mut buf: [i8; 15 + 3 * 4] = ::core::mem::uninitialized();
    procfdname(buf.as_mut_ptr(), fd as u32);
    syscall!(CHDIR, buf.as_ptr()) as i32
}

pub unsafe fn fchown(fd: i32, uid: uid_t, gid: gid_t) -> i32 {
    use fcntl::F_GETFD;
    let r = syscall!(FCHOWN, fd, uid, gid) as i32;
    if r != -EBADF || (syscall!(FCNTL, fd, F_GETFD) as i32) < 0 {
        return r;
    }
    let mut buf: [i8; 15 + 3 * 4] = ::core::mem::uninitialized();
    procfdname(buf.as_mut_ptr(), fd as u32);
    syscall!(CHOWN, buf.as_ptr(), uid, gid) as i32
}

pub unsafe fn fchownat(fd: i32, path: *const i8, uid: uid_t, gid: gid_t, flag: i32) -> i32 {
    syscall!(FCHOWNAT, fd, path, uid, gid, flag) as i32
}

pub unsafe fn fdatasync(fd: i32) -> i32 {
    syscall!(FDATASYNC, fd) as i32
}

pub unsafe fn fsync(fd: i32) -> i32 {
    syscall!(FSYNC, fd) as i32
}

pub unsafe fn ftruncate(fd: i32, length: isize) -> i32 {
    syscall!(FTRUNCATE, fd, length) as i32
}

/*
pub unsafe getcwd(buf: *mut i8, size: usize) {
    // TODO: Implement.
}
*/

pub unsafe fn getegid() -> gid_t {
    syscall!(GETEGID) as gid_t
}

pub unsafe fn geteuid() -> uid_t {
    syscall!(GETEUID) as uid_t
}

pub unsafe fn getgid() -> gid_t {
    syscall!(GETGID) as gid_t
}

pub unsafe fn getgroups(count: i32, list: *mut gid_t) -> i32 {
    syscall!(GETGROUPS, count, list) as i32
}

/*
pub unsafe fn gethostname(name: *mut i8, len: usize) -> i32 {
    TODO: Implement
}
*/

pub unsafe fn getpid() -> pid_t {
    syscall!(GETPID) as pid_t
}

pub unsafe fn getppid() -> pid_t {
    syscall!(GETPPID) as pid_t
}
