mod ffi;
use std::ffi::CString;

fn main() {
    println!("Hello, world!");

    unsafe {
        let result = ffi::add_number(10, 20);
        println!("10 + 20 = {}", result);

        let name = CString::new("Rust Developer").unwrap();
        let greeting_ptr = ffi::greet(name.as_ptr());
        let greeting = std::ffi::CStr::from_ptr(greeting_ptr).to_string_lossy();
        println!("{}", greeting);
    }
}
