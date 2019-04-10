#![feature(custom_attribute, lang_items, panic_info_message)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

mod format;
mod lang;
mod stack;

const BUF_SIZE: usize = 256;


fn non_reachable_function() {
    println!("This function should not be reached!\n");
}
