#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cascade::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate cascade;
use core::panic::PanicInfo;
use cascade::{
    println,
};

//entry point!
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    cascade::init();


    #[cfg(test)]
    test_main();

    println!("Hello World{}", "!");

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cascade::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
