#[cfg(not(all(target_family = "wasm", target_vendor = "unknown")))]
compile_error!("This crate is intended for use with the wasm32-unknown-unknown target only.");

pub mod asynch;
pub mod critical_section;
pub mod ffi;
pub mod io;
pub(crate) mod panic;
pub mod rng;
pub mod widget;

macro_rules! syscalls {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $fn_ident:ident ( $( $arg:ident : $arg_ty:ty ),* $(,)? ) $( -> $ret_ty:ty )?;
        )*
    ) => {
        #[link(wasm_import_module = "__xenon_syscall")]
        extern "C" {
            $(
                $(#[$meta])*
                $vis fn $fn_ident($($arg: $arg_ty),*) $(-> $ret_ty)?;
            )*
        }
    }
}

use asynch::executor::{Executor, Spawner};
use static_cell::StaticCell;

pub use embedded_graphics;
pub(crate) use syscalls;

#[no_mangle]
extern "C" fn __xenon_start() {
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();

    std::panic::set_hook(Box::new(panic::panic_hook));
    let executor = EXECUTOR.init(Executor::new());

    extern "Rust" {
        fn main(spawner: Spawner);
    }

    executor.start(|spawner| unsafe {
        main(spawner);
    });
}
