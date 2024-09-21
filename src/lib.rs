#[cfg(not(all(target_family = "wasm", target_vendor = "unknown")))]
compile_error!("This crate is intended for use with the wasm32-unknown-unknown target only.");

pub mod ffi;
pub mod widget;
pub mod io;
pub mod rng;
pub(crate) mod panic;

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

#[no_mangle]
extern "C" fn __xenon_start() {
    extern "Rust" {
        fn main();
    }

    std::panic::set_hook(Box::new(panic::panic_hook));

    unsafe {
        main();
    }
}

pub(crate) use syscalls;

pub use embedded_graphics;
