use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::kfree;
use std::os::kernel::loff_t;

#[repr(C)]
struct SomeData {
    data: [u8; 10],
    flag_which_is_never_set: bool,
}

static mut OTHER_DATA: Option<*const c_void> = None;

#[no_mangle]
pub fn rust_slab_read(_fps: *mut file,
                      buf: *mut c_void,
                      len: usize,
                       offset: *mut loff_t) -> isize {
    0
}

                       
#[no_mangle]
pub fn rust_slab_write(_fps: *mut file,
                       buf: *const c_void,
                       len: usize,
                       offset: *mut loff_t) -> isize {
    0
}

unsafe fn slab_operate_with_other_data() {
    match OTHER_DATA {
        None => OTHER_DATA = Some(::kzalloc(std::mem::size_of::<SomeData>(),
                                            ::GFP_KERNEL)),
        Some(p) => { kfree(p); OTHER_DATA = None }, 
    }
}
