#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Reduce warnings

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");

    let cool: CoolNumber = unsafe {
        my_cool_number(69)
    };

    println!("CoolNumber: {:?}", cool);

}
