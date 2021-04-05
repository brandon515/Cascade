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
pub mod memory;

use core::panic::PanicInfo;
use cpu::{
    exit_qemu,
    QemuExitCode,
};


pub fn init(){
    gdt::init();
    interrupts::init_idt();
    unsafe{ interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{
    BootInfo,
    entry_point,
};

#[cfg(test)]
entry_point!(kernel_main);

#[cfg(test)]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop()
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
    hlt_loop()
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	test_panic_handler(info)
}
