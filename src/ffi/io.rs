use std::fmt;

use crate::syscalls;

pub type Handle = u32;

syscalls! {
    pub fn print(ptr: *const u8, len: usize, new_line: bool);
    pub fn eprint(ptr: *const u8, len: usize, new_line: bool);
    pub fn log(level: u8, ptr: *const u8, len: usize);
}
