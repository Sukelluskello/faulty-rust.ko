static mut UNSIGED_COUNTER: u8 = 250;
static mut SIGNED_COUNTER: i8 = -124;

#[no_mangle]
pub fn rust_unsigned_overflow_read(_fps: *mut file,
                                   buf: *mut c_void,
                                   len: usize,
                                   offset: *mut loff_t) -> isize {

    let s = std::format!("Rust-Faulty: Overflow - Counter value: {}",
                         UNSIGNED_COUNTER);
    let n;
    
    unsafe {
        let ptr = ::kmalloc(s.len() + 1, GPF_KERNEL);
        core::ptr::copy(s.as_ptr(), ptr as *mut u8, s.len());
        core::ptr::write(ptr.offset(s.len() as isize), 0);

        n = std::os::kernel::simple_read_from_buffer(buf, len, offset, ptr,
                                                     std::os::kernel::strlen(ptr));
        std::os::kernel::kfree(ptr);
    }
        
    unsafe {
        UNSIGNED_COUNTER =+1;
    }

    if UNSIGNED_COUNTER == 1 {
        ::non_reachable_function();
    }

    n       
}

pub fn rust_signed_underflow_read(_fps: *mut file,
                        buf: *mut c_void,
                        len: usize,
                                  offset: *mut loff_t) -> isize {
    0
}

/*
	char *buffer = kmalloc(BUF_SIZE, GFP_KERNEL);
	ssize_t n = 0;

	// FAULT: signed underflow
	snprintf(buffer, BUF_SIZE, "Rust-Faulty: Underflow - Counter value :%d\n",
		signed_counter--); // note the behaviour of counter

	if (signed_counter == 126)
		non_reachable_function();

	n =  simple_read_from_buffer(buf, len, offset, buffer,
				       strlen(buffer));
	kfree(buffer);
	return n;
*/
