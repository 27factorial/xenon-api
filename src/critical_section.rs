use critical_section as cs;

use crate::ffi;

pub struct CriticalSection;
cs::set_impl!(CriticalSection);

unsafe impl cs::Impl for CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        ffi::asynch::cs_acquire()
    }

    unsafe fn release(_restore_state: critical_section::RawRestoreState) {
        ffi::asynch::cs_release()
    }
}

