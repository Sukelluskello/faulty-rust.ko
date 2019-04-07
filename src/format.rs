use std::os::raw::c_char;
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

#[no_mangle] // TODO use char
pub fn rust_format_write(_fps: *mut file,
                         buf: *const c_char,
                         len: usize,
                         offset: *mut loff_t) -> isize {

    let mut buffer: [c_char; ::BUF_SIZE] = [0; ::BUF_SIZE];
    let n;
    unsafe {
        n = std::os::kernel::simple_write_to_buffer(
            buffer.as_mut_ptr() as *mut c_void, ::BUF_SIZE,
            offset, buf as *const c_void, len);
    }
    buffer[n as usize] = '\0' as i8; // cast to usize exposes a latent bug

    // bindings for macros don't seem to work? (pr_info)
    unsafe {
        std::os::kernel::printk(buffer.as_ptr());
    }
    n
}

