#![no_std]
#![no_main]
extern crate alloc;
use kos::println;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, world!");
    return 0;
}
