#![allow(non_camel_case_types)]
pub type time_t = i64;
pub type suseconds_t = i64;

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
