use std::os::raw::c_void;
use std::os::kernel::file;
use std::os::kernel::kfree;
use std::os::kernel::loff_t;

use std::os::kernel::strlen;

#[repr(C)]
struct SomeData {
    data: [u8; 10],
    flag_which_is_never_set: bool,
}

static mut USER_CONTROLLED: Option<*const c_void> = None;
static mut OTHER_DATA: Option<*const c_void> = None;

#[no_mangle]
pub unsafe fn rust_slab_read(
    _fps: *mut file,
    buf: *mut c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    operate_with_other_data();
    match USER_CONTROLLED {
        None => {
            println!("Rust-Faulty: Slab - Read, no data\n");
            0
        },
        Some(p) => {
            println!("Rust-Faulty: Slab - Read, there is data\n");
            std::os::kernel::simple_read_from_buffer(
                buf,
                len,
                offset,
                p as *const c_void,
                strlen(p as *const i8) as usize,
            )
        }
    }
}

                       
#[no_mangle]
pub unsafe fn rust_slab_write(
    _fps: *mut file,
    buf: *const c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    operate_with_other_data();

    match USER_CONTROLLED {
        None => {
            println!("Rust-Faulty: Slab - Write, no data.\n");
        },
        Some(p) => {
            println!("Rust-Faulty: Slab - Write, free old data.\n");
            kfree(p);
        }
    }
    USER_CONTROLLED = Some(::kmalloc(std::mem::size_of::<SomeData>(),
                                     ::GFP_KERNEL));
    let p = USER_CONTROLLED.unwrap();
    let mut sd: SomeData = std::mem::transmute_copy(&p);
    if sd.flag_which_is_never_set {
        ::non_reachable_function();
    }

    std::os::kernel::simple_write_to_buffer(
        sd.data.as_mut_ptr() as *mut c_void,
        len,
        offset,
        buf,
        len,
    )
    
}

unsafe fn operate_with_other_data() {
    match OTHER_DATA {
        None => OTHER_DATA = Some(::kzalloc(std::mem::size_of::<SomeData>(),
                                            ::GFP_KERNEL)),
        Some(p) => { kfree(p); OTHER_DATA = None }, 
    }
}
