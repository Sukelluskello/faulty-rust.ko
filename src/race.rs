use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::loff_t;

static mut RACE1: *const c_void = core::ptr::null();
static mut RACE2: *const c_void = core::ptr::null();

#[no_mangle]
pub unsafe fn race_init() {
    RACE1 = ::kzalloc(::PAGE_SIZE, ::GFP_KERNEL);
    RACE2 = ::kzalloc(::PAGE_SIZE, ::GFP_KERNEL);
}

#[no_mangle]
pub unsafe fn race_exit() {
    std::os::kernel::kfree(RACE1);
    std::os::kernel::kfree(RACE2);
}

#[no_mangle]
pub unsafe fn rust_race_read(
    _fps: *mut file,
    buf: *mut c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    if std::os::kernel::strcmp(RACE1 as *const i8, RACE2 as *const i8) != 0 {
        ::non_reachable_function();
    }
    std::os::kernel::simple_read_from_buffer(
        buf,
        len,
        offset,
        RACE1,
        std::os::kernel::strlen(RACE1 as *const i8) as usize,
    )
}

#[no_mangle]
pub fn rust_race_write(
    _fps: *mut file,
    buf: *const c_char,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    let mut buffer: [c_char; ::PAGE_SIZE] = [0; ::PAGE_SIZE];
    let n;
    unsafe {
        n = std::os::kernel::simple_write_to_buffer(
            buffer.as_ptr() as *mut c_void,
            ::PAGE_SIZE,
            offset,
            buf as *const c_void,
            len,
        );
        core::ptr::write(buffer.as_mut_ptr().offset(n), 0);
        std::os::kernel::memcpy(
            RACE1 as *mut c_void,
            buffer.as_mut_ptr() as *mut c_void,
            len,
        );
        std::os::kernel::__udelay(1000); // TODO check why udelay binding was
        // not generated?
        std::os::kernel::memcpy(
            RACE2 as *mut c_void,
            buffer.as_mut_ptr() as *mut c_void,
            len,
        );
    }    
    n
}
