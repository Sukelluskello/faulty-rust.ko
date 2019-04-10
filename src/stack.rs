use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

static BUFFER: &str = "Write more than 10 bytes here to cause stack buffer overflow.\n";

#[no_mangle]
pub fn rust_stack_read(_fps: *mut file,
                        buf: *mut c_void,
                        len: usize,
                        offset: *mut loff_t) -> isize {
    unsafe {
        std::os::kernel::simple_read_from_buffer(buf, len, offset,
                                                 BUFFER.as_ptr() as *const c_void,
                                                 BUFFER.len())
    }
}
                       
#[no_mangle]
pub fn rust_stack_write(_fps: *mut file,
                         buf: *const c_char,
                         len: usize,
                        offset: *mut loff_t) -> isize {
    const KBUF_SIZE: usize = 10;
    let flag = 0; // variable to clobber
    let mut kbuf = [0; KBUF_SIZE];
    let bytes_written;

    unsafe {
        bytes_written = std::os::kernel::simple_write_to_buffer(
            kbuf.as_mut_ptr() as *mut c_void, len,
            offset, buf as *const c_void, len);
    }
    
    if flag != 0 {
        ::non_reachable_function();
    }
    
    bytes_written
}
