use crate::syscalls;

syscalls! {
    pub fn random_32() -> u32;
    pub fn random_64() -> u64;
    pub fn random_bytes(ptr: *mut u8, len: usize);
}