#![no_std]

#[macro_use]
extern crate syscall;

extern crate errno_const;

pub mod unistd;
pub mod sys;
pub mod fcntl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn getpid() {
        use unistd::getpid;
        assert!(unsafe { getpid() } > 1);
    }
}
