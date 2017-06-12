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

//julia> x = ccall((:addition, "libmy_dylib.dylib"), UInt32, (UInt32,UInt32), 2,2)
//0x00000004

//julia> Int(x)
//4

//julia> function rust_addition(a::Int, b::Int)
//      val = ccall((:addition, "libmy_dylib.dylib"), UInt32, (UInt32,UInt32), a,b)
//     println(val)
//    end
//rust_addition (generic function with 1 method)

//julia> rust_addition(1,2)



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