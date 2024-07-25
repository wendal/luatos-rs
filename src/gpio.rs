use core::ffi::{c_int, c_void};

extern "C" {
    fn luat_gpio_set(pin: c_int, value: c_int) -> c_void;
    fn luat_gpio_get(pin: c_int) -> c_int;
    fn luat_gpio_close(pin: c_int) -> c_void;
}

pub fn pin_set(pin: i32, level: i32) -> () {
    unsafe {
        luat_gpio_set(pin, level);
    }
}

pub fn pin_get(pin: i32) -> i32 {
    unsafe {
        luat_gpio_get(pin)
    }
}

pub fn pin_close(pin: i32) -> () {
    unsafe {
        luat_gpio_close(pin);
    }
}

// 输出功能
pub struct OutPut {
    id: i32
}

impl OutPut {
    pub fn set(self, level: i32) -> () {
        pin_set(self.id, level);
    }

    pub fn close(self) -> () {
        pin_close(self.id)
    }
}

// 输入功能

pub struct InPut {
    id: i32
}

impl InPut {
    pub fn get(self) -> i32 {
        pin_get(self.id)
    }

    pub fn close(self) -> () {
        pin_close(self.id)
    }
}

pub struct GPIO {

}

pub struct GpioConfig {
    id: i32,
    pull: i32,
    alt: i32
}

impl GPIO {
    pub fn output(cfg: &GpioConfig, level: i32) -> OutPut {
        OutPut {id: cfg.id}
    }
    pub fn input(cfg: &GpioConfig) -> InPut {
        InPut {id: cfg.id}
    }
}