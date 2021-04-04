#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cascade::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate cascade;
use bootloader::{
    BootInfo,
    entry_point,
};
use core::panic::PanicInfo;
use cascade::{
    println,
};

//entry point!
entry_point!(kernel_main); //this macro checks to make sure the entry point is the correct function signature

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    cascade::init();


    #[cfg(test)]
    test_main();

    println!("Hello World{}", "!");

    cascade::hlt_loop();
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
    cascade::hlt_loop();
}
