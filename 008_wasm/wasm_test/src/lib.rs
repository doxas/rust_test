
use std::ffi::CString;
use std::os::raw::c_char;

static TEST_STRING: &'static str = "test string from rust";

#[no_mangle]
pub fn getstring() -> *mut c_char {
    let s = CString::new(TEST_STRING).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn getstringlength() -> usize {
    TEST_STRING.len()
}

#[no_mangle]
pub fn sumint(x: i32, y: i32) -> i32 {
    println!("{}, {}", x, y);
    x + y
}

#[no_mangle]
pub fn sumfloat(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
pub fn sumsize(x: isize, y: isize) -> isize {
    x + y
}

