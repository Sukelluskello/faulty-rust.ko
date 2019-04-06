#![feature(custom_attribute, lang_items, panic_info_message)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

// Defines various language items that need to be around
mod lang;

const ROOT: &'static str = "rfaulty";

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");
    init_endpoints();
}

fn init_endpoints() {
    let _root = c_str!("rfaulty");
    /*
    unsafe {
        std::os::kernel::debugfs_create_dir(root, 0);
    }*/
}
