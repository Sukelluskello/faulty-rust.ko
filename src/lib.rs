#![feature(custom_attribute, lang_items, panic_info_message)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

use std::os::kernel::__kmalloc;
use std::os::raw::c_void;
use std::os::kernel::gfp_t;

mod format;
mod lang;
mod slab;
mod stack;


//
// Redefinitions of Kernel constants
//
// We would want to generate these automatically
// See e.g.
// http://vojtech.kral.hk/en/rust-importing-c-constants-proof-of-concept/
//
// TODO: use enums?
//
const ___GFP_IO: gfp_t = 0x40;
const ___GFP_FS: gfp_t = 0x80;
const ___GFP_ZERO: gfp_t = 0x8000;
const ___GFP_DIRECT_RECLAIM: gfp_t = 0x200000;
const ___GFP_KSWAPD_RECLAIM: gfp_t = 0x400000;

const __GFP_RECLAIM: gfp_t = ___GFP_DIRECT_RECLAIM | ___GFP_KSWAPD_RECLAIM;
const __GFP_IO: gfp_t = ___GFP_IO;
const __GFP_FS: gfp_t = ___GFP_FS;
const __GFP_ZERO: gfp_t = ___GFP_ZERO;
const GFP_KERNEL: gfp_t = __GFP_RECLAIM | __GFP_IO | __GFP_FS;

// Redefinitions of static inline functions
//
// Rust-bindgen cannot currently generate bindings for static inline
// functions
// See https://github.com/rust-lang/rust-bindgen/issues/1090

unsafe fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void {
    __kmalloc(size, flags | __GFP_ZERO)
}

unsafe fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void {
    __kmalloc(size, flags)
}

// local definitions
const BUF_SIZE: usize = 256;


fn non_reachable_function() {
    println!("This function should not be reached!\n");
}
