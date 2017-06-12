extern crate libc;
use libc::uint32_t;

#[no_mangle]
pub extern "C" fn hello()  {
    println!("Hello from Rust!");
}
#[no_mangle]
pub extern "C" fn print_number(x: i32) {
    println!("x is: {}", x);
}



#[no_mangle]
pub extern "C" fn addition(a: uint32_t, b: uint32_t) -> uint32_t {
    let c = a + b;
    println!("Sum : {}", c);
    
    return a + b
}



#[allow(dead_code)]
pub extern "C" fn fix_linking_when_not_using_stdlib() {
    panic!()
}