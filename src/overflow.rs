use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

static mut UNSIGNED_COUNTER: u8 = 250;
static mut SIGNED_COUNTER: i8 = -124;

#[no_mangle]
pub fn rust_unsigned_overflow_read(_fps: *mut file,
                                   buf: *mut c_void,
                                   len: usize,
                                   offset: *mut loff_t) -> isize {
        
    unsafe {
        UNSIGNED_COUNTER +=1;

        if UNSIGNED_COUNTER == 1 {
            ::non_reachable_function();
        }
    }

    let s = "Rust-Faulty: Overflow";

    unsafe {
        write_to_buffer(s, buf, len, offset)
    }
}

#[no_mangle]
pub fn rust_signed_underflow_read(_fps: *mut file,
                        buf: *mut c_void,
                        len: usize,
                                  offset: *mut loff_t) -> isize {

    unsafe {
        SIGNED_COUNTER =-1;

        if SIGNED_COUNTER == 126 {
        ::non_reachable_function();
        }
    }
    
    let s = "Rust-Faulty: Underflow";    

    unsafe {
        write_to_buffer(s, buf, len, offset)
    }
}


unsafe fn write_to_buffer(s: &str,
                          buf: *mut c_void,
                          len: usize,
                          offset: *mut loff_t) -> isize {
    let ptr = ::kmalloc(s.len() + 1, ::GFP_KERNEL) as *mut c_char;
    core::ptr::copy(s.as_ptr(), ptr as *mut u8, s.len());
    core::ptr::write(ptr.offset(s.len() as isize), 0);

    let n = std::os::kernel::simple_read_from_buffer(buf, len, offset,
                                                     ptr as *mut c_void,
                                                     s.len() + 1);
    std::os::kernel::kfree(ptr as *mut c_void);

    n
}
