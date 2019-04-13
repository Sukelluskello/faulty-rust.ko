use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

static BUFFER: &str = "just some small data buffer\n";

#[no_mangle]
pub unsafe fn rust_use_after_free_read(
    _fps: *mut file,
    buf: *mut c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    let ptr = ::kmalloc(len, ::GFP_KERNEL) as *mut i8;
    std::os::kernel::strncpy(ptr, BUFFER.as_ptr() as *const i8, len as u64);
    std::os::kernel::kfree(ptr as *const c_void);
    // copy_to_user is static inline and not automatically generated
    // we'll use simple_read_from_buffer here instead of it
    std::os::kernel::simple_read_from_buffer(
        buf,
        len,
        offset,
        ptr as *const c_void,
        len,
    );
    len as isize
}
