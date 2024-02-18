#![no_std]

extern "C" {
    fn print_message(chars: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn rust_msg() {
    let msg = "Hello, world!\n";
    let len = msg.len();
    unsafe {
        print_message(msg.as_ptr(), len);
    }
}
