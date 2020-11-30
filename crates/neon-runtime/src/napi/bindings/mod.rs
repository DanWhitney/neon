macro_rules! napi_name {
    (typeof_value) => {
        "napi_typeof"
    };
    ($name:ident) => {
        concat!("napi_", stringify!($name))
    };
}

macro_rules! napi_bindings {
    (extern "C" {
        $(fn $name:ident($($param:ident: $ptype:ty$(,)?)*) -> $rtype:ty;)+
    }) => {
        pub(crate) struct Napi {
            $(
                $name: unsafe extern "C" fn(
                    $($param: $ptype,)*
                ) -> $rtype,
            )*
        }

        pub(crate) unsafe fn load() {
            let host = Library::this();
            #[cfg(windows)]
            let host = host.unwrap();

            NAPI = Napi {
                $(
                    $name: *host.get(napi_name!($name).as_bytes()).unwrap(),
                )*
            };
        }

        $(
            #[inline]
            pub(crate) unsafe fn $name($($param: $ptype,)*) -> $rtype {
                (NAPI.$name)($($param,)*)
            }
        )*

        fn panic_load<T>() -> T {
            panic!("Must load N-API bindings")
        }

        static mut NAPI: Napi = {
            $(
                unsafe extern "C" fn $name($(_: $ptype,)*) -> $rtype {
                    panic_load()
                }
            )*

            Napi {
                $(
                    $name,
                )*
            }
        };
    };
}

use std::sync::Once;

pub(crate) use functions::*;

mod types;
mod functions;

static SETUP: Once = Once::new();

pub fn setup() {
    SETUP.call_once(|| unsafe {
        load();
    });
}
