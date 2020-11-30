//! Facilities for working with Array `value`s.

use crate::raw::{Env, Local};

use crate::napi::bindings as napi;

pub unsafe extern "C" fn new(out: &mut Local, env: Env, length: u32) {
    assert_eq!(
        napi::create_array_with_length(env, length as usize, out as *mut _),
        napi::Status::Ok,
    );
}

/// Gets the length of a `value` containing a JavaScript Array.
///
/// # Panics
/// This function panics if `array` is not an Array, or if a previous n-api call caused a pending
/// exception.
pub unsafe extern "C" fn len(env: Env, array: Local) -> u32 {
    let mut len = 0;
    assert_eq!(
        napi::get_array_length(env, array, &mut len as *mut _),
        napi::Status::Ok
    );
    len
}
