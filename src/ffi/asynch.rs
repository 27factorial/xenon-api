use crate::syscalls;

syscalls! {
    pub fn cs_acquire();
    pub fn cs_release();
    pub fn wait();
    pub fn register_timer_wake(wake: extern "C" fn(), micros: u64);
    pub fn register_io_wake(wake: extern "C" fn(), readable: bool, writable: bool);
    pub fn resume();
}
