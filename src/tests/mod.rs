#[allow(unused_imports)]
use crate::{
    //println,
    serial_println,
    serial_print,
    cpu::{
        exit_qemu,
        QemuExitCode,
    }
};
use core::panic::PanicInfo;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("\nRunning {} tests\n", tests.len());
    for t in tests {
        t();
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

