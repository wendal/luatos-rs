use core::ffi::{c_void, c_uchar, c_uint};
use core::fmt;

use alloc::fmt::format;
use alloc::string::String;

extern "C" {
    fn luat_log_write(data: *const c_uchar, len: c_uint) -> c_void;
}

pub fn write(msg: &str) -> () {
    if msg.len() > 0 {
        unsafe {
            luat_log_write(msg.as_ptr(), msg.len() as u32);
        }
    }
}

pub struct LogOut {

}

impl fmt::Write for LogOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(s);
        Ok(())
    }
}

impl LogOut {
    pub fn debug(self, s: String) -> () {
        write(s.as_str());
    }
}

// pub static LOGGER: LogOut = LogOut {};