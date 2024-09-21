use std::panic::PanicHookInfo;

use crate::{ffi, syscalls};

syscalls! {
    fn panic(ptr: *const u8, len: usize);
}

pub(crate) fn panic_hook(payload: &PanicHookInfo<'_>) {
    let panic_message = format!("{}", payload);
    unsafe { panic(panic_message.as_ptr(), panic_message.len()) }
}
