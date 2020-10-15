#![feature(
    no_core, start, lang_items, box_syntax, never_type, linkage,
    extern_types, thread_local
)]
#![no_core]

extern crate mini_core;

use mini_core::*;
use mini_core::libc::*;

#[lang = "start"]
fn start(
    main: fn(),
    argc: isize,
    argv: *const *const u8,
) -> isize {
    if argc == 3 {
        unsafe { puts(*argv as *const i8); }
        unsafe { puts(*((argv as usize + 0) as *const *const i8)); }
    }

    0
}

fn main() {}
