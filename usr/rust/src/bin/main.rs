#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

#[no_mangle]
pub fn main() {
    println!("Hello, world!");
    for j in 0..1000 {
    }

    println!("end user main");
}