#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
pub mod panic;
pub mod vga;
pub mod cpu;
pub mod serial;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;
use cpu::{
    exit_qemu,
    QemuExitCode,
};


pub fn init(){
    gdt::init();
    interrupts::init_idt();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

/*#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tests::test_panic_handler(info)
}*/

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(), //this ensures that everything we run has the Fn trait, which means it's a function
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} tests\n", tests.len());
    for t in tests {
        t.run();
    }
    serial_println!();
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	test_panic_handler(info)
}
