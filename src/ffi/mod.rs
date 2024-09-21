pub mod io;
pub mod misc;
pub mod net;
pub mod time;
pub mod tuple;
pub mod widget;
pub mod rng;

trait Sealed {}

#[expect(
    private_bounds,
    reason = "this trait is sealed and cannot be implemented outside of this crate."
)]
pub trait WasmType: FfiSafe + Sealed {}

/// # Safety
///
/// `Self` must be valid to pass across the C function ABI.
pub unsafe trait FfiSafe: Sized {}

pub trait IntoFfi: FromFfi + Sized {
    fn into_ffi(self) -> <Self as FromFfi>::Ffi;
}

pub trait FromFfi: Sized {
    type Ffi: FfiSafe;

    fn from_ffi(ffi: Self::Ffi) -> Self;
}

mod impls {
    use std::ptr::NonNull;

    use super::{FfiSafe, Sealed, WasmType};

    macro_rules! wasm_type_prim {
        ($($t:ty),* $(,)?) => {
            $(
                impl WasmType for $t {}
                unsafe impl FfiSafe for $t {}
                impl Sealed for $t {}
            )*
        }
    }

    wasm_type_prim! {
        i8,
        u8,
        i16,
        u16,
        i32,
        u32,
        i64,
        u64,
        isize,
        usize,
        f32,
        f64,
        bool,
        char,
    }

    impl<T> WasmType for &T {}
    unsafe impl<T> FfiSafe for &T {}
    impl<T> Sealed for &T {}

    impl<T> WasmType for &mut T {}
    unsafe impl<T> FfiSafe for &mut T {}
    impl<T> Sealed for &mut T {}

    impl<T> WasmType for *const T {}
    unsafe impl<T> FfiSafe for *const T {}
    impl<T> Sealed for *const T {}

    impl<T> WasmType for *mut T {}
    unsafe impl<T> FfiSafe for *mut T {}
    impl<T> Sealed for *mut T {}

    impl<T> WasmType for NonNull<T> {}
    unsafe impl<T> FfiSafe for NonNull<T> {}
    impl<T> Sealed for NonNull<T> {}

    impl<T: WasmType, const N: usize> WasmType for [T; N] {}
    unsafe impl<T: FfiSafe, const N: usize> FfiSafe for [T; N] {}
    impl<T: Sealed, const N: usize> Sealed for [T; N] {}
}
