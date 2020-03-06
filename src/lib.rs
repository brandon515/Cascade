#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
pub mod panic;
pub mod vga;
pub mod tests;
pub mod cpu;
pub mod serial;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
use core::panic::PanicInfo;
#[cfg(test)]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    tests::test_panic_handler(info)
}
