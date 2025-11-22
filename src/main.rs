use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn add_number(a: c_int, b: c_int) -> c_int;
    fn greet(name: *const c_char) -> *const c_char;
}

fn main() {
    println!("Hello, world!");

    unsafe {
        let result = add_number(10, 20);
        println!("10 + 20 = {}", result);

        let name = CString::new("Rust Developer").unwrap();
        let greeting_ptr = greet(name.as_ptr());
        let greeting = std::ffi::CStr::from_ptr(greeting_ptr).to_string_lossy();
        println!("{}", greeting);
    }
}
