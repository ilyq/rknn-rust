use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    pub fn add_number(a: c_int, b: c_int) -> c_int;
    pub fn greet(name: *const c_char) -> *const c_char;
}
