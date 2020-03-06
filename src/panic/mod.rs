use core::panic::PanicInfo;
#[allow(unused_imports)]
use crate::{
    println,
    serial_print,
    serial_println,
    cpu::{
        QemuExitCode,
        exit_qemu,
    },
};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
