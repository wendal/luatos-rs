#![no_std]
use core::ffi::{c_uchar, c_uint};
use core::{ffi::c_void, panic::PanicInfo};

extern "C" {
    fn luat_log_write(data: *const c_uchar, len: c_uint) -> c_void;
}

#[no_mangle]
pub extern "C" fn luat_user_main(_a: c_void) -> i32 {
    unsafe {
        let msg = "hello world from RUST";
        luat_log_write("hello world from RUST".as_ptr(), msg.len() as u32);
    }
    0
}


#[panic_handler]
fn panic(_info:&PanicInfo) -> !{
    loop{}
}
