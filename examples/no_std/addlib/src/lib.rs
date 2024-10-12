#![no_std]
extern crate alloc;
use kos::print;

#[no_mangle]
pub extern "C" fn print_added(a: isize, b: isize) {
    print!("{}", a + b);
}

#[no_mangle]
pub extern "C" fn add_integers(a: isize, b: isize) -> isize {
    a + b
}
