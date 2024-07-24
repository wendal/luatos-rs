#![no_std]
use core::{ffi::c_void, panic::PanicInfo};

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn luat_user_main(_a: c_void) -> i32 {
    0
}


#[panic_handler]
fn panic(_info:&PanicInfo) -> !{
    loop{}
}
