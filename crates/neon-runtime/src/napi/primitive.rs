use raw::{Local, Env};

use nodejs_sys as napi;

/// Mutates the `out` argument provided to refer to the global `undefined` object.
pub unsafe fn undefined(out: &mut Local, env: Env) {
    napi::napi_get_undefined(env, out as *mut Local);
}

/// Mutates the `out` argument provided to refer to the global `null` object.
pub unsafe fn null(out: &mut Local, env: Env) {
    napi::napi_get_null(env, out as *mut Local);
}

/// Mutates the `out` argument provided to refer to one of the global `true` or `false` objects.
pub unsafe fn boolean(out: &mut Local, env: Env, b: bool) {
    napi::napi_get_boolean(env, b, out as *mut Local);
}

/// Get the boolean value out of a `Local` object. If the `Local` object does not contain a
/// boolean, this function panics.
pub unsafe fn boolean_value(env: Env, p: Local) -> bool {
    let mut value = false;
    assert_eq!(napi::napi_get_value_bool(env, p, &mut value as *mut bool), napi::napi_status::napi_ok);
    value
}

// DEPRECATE(0.2)
pub unsafe fn integer(_out: &mut Local, _isolate: Env, _x: i32) { unimplemented!() }

pub unsafe fn is_u32(_p: Local) -> bool { unimplemented!() }

pub unsafe fn is_i32(_p: Local) -> bool { unimplemented!() }

// DEPRECATE(0.2)
pub unsafe fn integer_value(_p: Local) -> i64 { unimplemented!() }

/// Mutates the `out` argument provided to refer to a newly created `Local` containing a
/// JavaScript number.
pub unsafe fn number(out: &mut Local, env: Env, v: f64) {
    napi::napi_create_double(env, v, out as *mut Local);
}

/// Gets the underlying value of an `Local` object containing a JavaScript number. Panics if
/// the given `Local` is not a number.
pub unsafe fn number_value(env: Env, p: Local) -> f64 {
    let mut value = 0.0;
    assert_eq!(napi::napi_get_value_double(env, p, &mut value as *mut f64), napi::napi_status::napi_ok);
    value
}
