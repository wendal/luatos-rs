use core::ffi::{c_int, c_uchar, c_void};


pub enum PullConfig {
    OpenDrain = 0,
    PullUp = 1,
    PullDown = 2
}

pub enum GpioLevel {
    LevelLow = 0,
    LevelHigh = 1
}



#[repr(C)]
struct LuatGpioCfg {
    pin: c_int,
    mode: c_uchar,
    pull: c_uchar,
    irq_type: c_uchar,
    output_level: c_uchar,
    irq_cb: *const c_uchar,
    irq_args: *const c_uchar,
    alt_fun: c_uchar
}

extern "C" {
    fn luat_gpio_set(pin: c_int, value: c_int) -> c_void;
    fn luat_gpio_get(pin: c_int) -> c_int;
    fn luat_gpio_close(pin: c_int) -> c_void;
    fn luat_gpio_set_default_cfg(cfg: *const LuatGpioCfg);
    fn luat_gpio_open(cfg: *const LuatGpioCfg) -> c_int;
}


pub fn pin_set(pin: i32, level: GpioLevel) -> () {
    unsafe {
        match level {
            GpioLevel::LevelHigh => {
                luat_gpio_set(pin, 1);
            }
            GpioLevel::LevelLow => {
                luat_gpio_set(pin, 0);
            }
        }
    }
}

pub fn pin_get(pin: i32) -> GpioLevel {
    unsafe {
        if luat_gpio_get(pin) == 0 {
            GpioLevel::LevelLow
        }
        else {
            GpioLevel::LevelHigh
        }
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
    pub fn set(self, level: GpioLevel) -> () {
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
    pub fn get(self) -> GpioLevel {
        pin_get(self.id)
    }

    pub fn close(self) -> () {
        pin_close(self.id)
    }
}

// pub struct GPIO {
// }

pub struct GpioConfig {
    pub id: i32,
    pub pull: i32,
    pub alt: i32
}

// impl GPIO {
    pub fn setup_output(cfg: &GpioConfig, level: GpioLevel) -> OutPut {
        let conf = LuatGpioCfg{
            pin: cfg.id,
            mode: 0,
            pull: cfg.pull as u8,
            irq_type: 0,
            output_level: level as u8,
            irq_cb: 0 as *const c_uchar,
            irq_args: 0 as *const c_uchar,
            alt_fun: cfg.alt as u8
        };
        unsafe {
            luat_gpio_set_default_cfg(&conf);
            let result = luat_gpio_open(&conf);
            if result == 0 {
                
            }
        }
        OutPut {id: cfg.id}
    }
    pub fn setup_input(cfg: &GpioConfig) -> InPut {
        let conf = LuatGpioCfg{
            pin: cfg.id,
            mode: 1,
            pull: cfg.pull as u8,
            irq_type: 0,
            output_level: 0,
            irq_cb: 0 as *const c_uchar,
            irq_args: 0 as *const c_uchar,
            alt_fun: cfg.alt as u8
        };
        unsafe {
            luat_gpio_set_default_cfg(&conf);
            let result = luat_gpio_open(&conf);
            if result == 0 {
                
            }
        }
        InPut {id: cfg.id}
    }
// }
