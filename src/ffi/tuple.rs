use paste::paste;

macro_rules! wasm_tuple {
    ($($t:ty),*; $n:tt) => {
        paste! {
            #[doc = concat!(
                "An FFI-safe tuple of ",
                stringify!($n),
                " ",
                wasm_tuple!(@plural $n),
                ".",
            )]
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
            pub struct [<Tuple $n>]<$($t),*>($(pub $t),*);

            unsafe impl<$($t),*> crate::ffi::FfiSafe for [<Tuple $n>]<$($t),*>
            where
                $(
                    $t: crate::ffi::FfiSafe
                ),*
            {}

            impl<$($t),*> crate::ffi::FromFfi for ($($t),* ,)
            where
            $(
                $t: crate::ffi::FfiSafe
            ),*
            {
                type Ffi = [<Tuple $n>]<$($t),*>;

                #[inline(always)]
                #[allow(non_snake_case)]
                fn from_ffi([<Tuple $n>]($($t),*): [<Tuple $n>]<$($t),*>) -> Self {
                    ($($t),* ,)
                }
            }

            impl<$($t),*> crate::ffi::IntoFfi for ($($t),* ,)
            where
            $(
                $t: crate::ffi::FfiSafe
            ),*
            {
                #[inline(always)]
                #[allow(non_snake_case)]
                fn into_ffi(self) -> <Self as crate::ffi::FromFfi>::Ffi {
                    let ($($t),* ,) = self;

                    [<Tuple $n>]($($t),*)
                }
            }

            impl<$($t),*> From<[<Tuple $n>]<$($t),*>> for ($($t),* ,) {
                #[inline(always)]
                #[allow(non_snake_case)]
                fn from([<Tuple $n>]($($t),*): [<Tuple $n>]<$($t),*>) -> Self {
                    ($($t),* ,)
                }
            }
        }
    };
    (@plural 1) => {
        "element"
    };
    (@plural $_n:literal) => {
        "elements"
    }
}

wasm_tuple!(T0; 1);
wasm_tuple!(T0, T1; 2);
wasm_tuple!(T0, T1, T2; 3);
wasm_tuple!(T0, T1, T2, T3; 4);
wasm_tuple!(T0, T1, T2, T3, T4; 5);
wasm_tuple!(T0, T1, T2, T3, T4, T5; 6);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6; 7);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7; 8);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8; 9);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; 10);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 11);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 12);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 13);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; 14);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; 15);
wasm_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; 16);
