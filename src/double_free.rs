use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::kfree;
use std::os::kernel::loff_t;

static mut DOUBLE_FREE: *const c_void = core::ptr::null();

#[no_mangle]
pub fn rust_df_alloc(
    _fps: *mut file,
    _buf: *mut c_void,
    len: usize,
    _offset: *mut loff_t,
) -> isize {
    unsafe {
        DOUBLE_FREE = ::kmalloc(len, ::GFP_KERNEL);
    }
    len as isize
}

#[no_mangle]
pub fn rust_df_free(
    _fps: *mut file,
    _buf: *const c_void,
    len: usize,
    _offset: *mut loff_t,
) -> isize {
    unsafe {
        kfree(DOUBLE_FREE);
    }
    len as isize    
}
