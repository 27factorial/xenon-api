use crate::syscalls;

syscalls! {
    pub fn clone_binary_data(id: i32) -> i32;
    pub fn drop_binary_data(id: i32);
}
