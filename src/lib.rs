#![no_std]

#[macro_use]
extern crate syscall;
pub mod unistd;
pub mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn getpid() {
        use unistd::getpid;
        assert!(getpid() > 1);
    }
}
