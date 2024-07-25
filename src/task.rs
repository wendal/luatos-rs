
use core::ffi::{c_void, c_uint};



extern "C" {
    fn luat_rtos_task_sleep(ms: c_uint) -> c_void;
}




pub fn sleep(ms: u32) -> () {
    unsafe {
        if ms > 0 {
            luat_rtos_task_sleep(ms);
        }
    }
}