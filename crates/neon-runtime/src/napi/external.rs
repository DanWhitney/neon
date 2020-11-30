use std::mem::MaybeUninit;

use crate::raw::{Env, Local};
use crate::napi::bindings as napi;

/// `finalize_external` is invoked immediately before a `External` is garbage collected
extern "C" fn finalize_external<T: Send + 'static>(
    env: Env,
    // Raw pointer to a `Box<T>` stored by a `External`
    data: *mut std::ffi::c_void,
    // Pointer to a Rust `fn` stored in the `hint` parameter of a `External` called
    // with the contents of `data` immediately before the value is garbage collected.
    hint: *mut std::ffi::c_void,
) {
    unsafe {
        let data = Box::<T>::from_raw(data as *mut _);
        let finalizer: fn(Env, T) = std::mem::transmute(hint as *const ());

        finalizer(env, *data);
    }
}

/// Returns a pointer to data stored in a `External`
/// Safety: `deref` must only be called with `External` created by that
/// module. Calling `deref` with an external created by another native module,
/// even another neon module, is undefined behavior.
/// https://github.com/neon-bindings/neon/issues/591
pub unsafe fn deref<T: Send + 'static>(
    env: Env,
    local: Local,
) -> Option<*const T> {
    let mut result = MaybeUninit::uninit();
    let status = napi::typeof_value(
        env,
        local,
        result.as_mut_ptr(),
    );

    assert_eq!(status, napi::Status::Ok);

    let result = result.assume_init();

    // Note: This only validates it is an external, not that it was created by
    // this module. In this future, this can be improved with type tagging:
    // https://nodejs.org/api/n-api.html#n_api_type_tag
    // https://github.com/neon-bindings/neon/issues/591
    if result != napi::ValueType::External {
        return None;
    }

    let mut result = MaybeUninit::uninit();
    let status = napi::get_value_external(
        env,
        local,
        result.as_mut_ptr(),
    );

    assert_eq!(status, napi::Status::Ok);

    Some(result.assume_init() as *const _)
}

/// Creates a `External` from a Rust type
pub unsafe fn create<T: Send + 'static>(
    env: Env,
    v: T,
    finalizer: fn(Env, T),
) -> Local {
    let v = Box::new(v);
    let mut result = MaybeUninit::uninit();

    let status = napi::create_external(
        env,
        Box::into_raw(v) as *mut _,
        Some(finalize_external::<T>),
        // Casting to `*const ()` is required to ensure the correct layout
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html
        finalizer as *const () as *mut _,
        result.as_mut_ptr(),
    );

    // `create_external` will only fail if the VM is in a throwing state
    // or shutting down.
    assert_eq!(status, napi::Status::Ok);

    result.assume_init()
}
