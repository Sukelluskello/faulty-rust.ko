use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

static SOME_STRING: &str = "A write to this endpoint will get copied to kernel message buffer\n";

#[no_mangle]
pub fn rust_format_read(_fps: *mut file,
                        buf: *mut c_void,
                        len: usize,
                        offset: *mut loff_t) -> isize {
    unsafe {
        std::os::kernel::simple_read_from_buffer(buf, len, offset,
                                                 SOME_STRING.as_ptr() as *const c_void,
                                                 SOME_STRING.len())
    }
}

#[no_mangle]
pub fn rust_format_write() {
    
}
