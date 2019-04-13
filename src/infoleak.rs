use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

// infoleak was struct in c-version, here just a buffer
static mut UNINITIALIZED: *const c_void = core::ptr::null();

#[no_mangle]
pub unsafe fn infoleak_init() {
    UNINITIALIZED = ::kmalloc(::BUF_SIZE, ::GFP_KERNEL);
}

#[no_mangle]
pub unsafe fn infoleak_exit() {
    std::os::kernel::kfree(UNINITIALIZED);
}

#[no_mangle]
pub fn rust_infoleak_read(
    _fps: *mut file,
    buf: *mut c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    let n = if len < ::BUF_SIZE { len } else { ::BUF_SIZE };
    unsafe {
        std::os::kernel::simple_read_from_buffer(
            buf,
            len,
            offset,
            UNINITIALIZED,
            n,
        )
    }
}
