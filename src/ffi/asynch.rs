use crate::syscalls;

syscalls! {
    pub fn cs_acquire();
    pub fn cs_release();
    pub fn wait();
    pub fn resume();
}
