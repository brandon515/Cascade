#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cascade::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate cascade;
use cascade::{
    println,
};

//entry point!
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

