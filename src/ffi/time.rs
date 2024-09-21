use crate::syscalls;

syscalls! {
    pub fn get_time() -> u64;
}

#[repr(C)]
pub struct Instant(u64);

impl Instant {
    pub fn now() -> Self {
        let micros = unsafe { get_time() };

        Self(micros)
    }
}

#[repr(C)]
pub struct Duration(u64);
