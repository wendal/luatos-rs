#![no_std]


extern crate core;
#[macro_use]
extern crate alloc;

use core::ffi::c_void;
use core::panic::PanicInfo;
use alloc::vec::Vec;

mod talloc;
mod gpio;
mod log;
mod task;

#[no_mangle]
pub extern "C" fn luat_user_main(_a: c_void) -> i32 {
    let mut i: i32 = 0;
    let mut table : Vec<i32> = Vec::new();
    while i < 10000000 {
        table.push(i);
        if (i % 2) == 0 {
            let logger = log::LogOut {};
            logger.debug(format!("Hello world from rust! ABCDEFG {}", i));
        }
        i += 1;
        gpio::pin_set(27, if i % 2 == 0 {1} else {0});
        gpio::pin_get(23);
        task::sleep(1000);
        table.pop();
    }
    0
}


