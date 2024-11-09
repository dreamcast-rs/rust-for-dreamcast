#![no_main]
extern crate kos;

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, world from Rust!");

    0
}
