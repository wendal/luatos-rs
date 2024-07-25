#![no_std]


extern crate core;
#[macro_use]
extern crate alloc;
use core::panic::PanicInfo;

mod talloc;
mod gpio;
mod log;
mod task;

//-----------------------------------------------------------------------------
// 以下是 heap分配和pain处理器的代码

#[global_allocator]
static ALLOCATOR: talloc::DefaultAllocator = talloc::DefaultAllocator;

#[panic_handler]
fn panic(info:&PanicInfo) -> !{
    let logger = log::LogOut {};
    logger.debug(format!("{info}"));
    loop {}
}
