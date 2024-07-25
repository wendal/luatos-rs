use core::ffi::{c_int, c_uint, c_void};


extern "C" {
    // 经典分配函数
    fn luat_heap_malloc(size: c_uint) -> *const c_void;
    fn luat_heap_free(ptr: *const c_void) -> c_void;
    // 按区域划分的分配函数
    fn luat_heap_opt_malloc(tp: c_int, size: c_uint) -> *const c_void;
    fn luat_heap_opt_free(tp: c_int, ptr: *const c_void) -> c_void;
}

// 内存分配函数, 使用默认实现
pub struct DefaultAllocator;

unsafe impl core::alloc::GlobalAlloc for DefaultAllocator {

    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = unsafe { luat_heap_malloc(layout.size() as u32) } as *mut u8;
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { luat_heap_free(ptr as *const c_void) };
    }
}

// PSRAM分配器， 只使用PSRAM
pub struct PsramAllocator;
unsafe impl core::alloc::GlobalAlloc for PsramAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = unsafe { luat_heap_opt_malloc(2, layout.size() as u32) } as *mut u8;
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { luat_heap_opt_free(2, ptr as *const c_void) };
    }
}


// PSRAM分配器， 只使用PSRAM
pub struct SramAllocator;
unsafe impl core::alloc::GlobalAlloc for SramAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = unsafe { luat_heap_opt_malloc(1, layout.size() as u32) } as *mut u8;
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { luat_heap_opt_free(1, ptr as *const c_void) };
    }
}