/*
 * This tests to make sure the global descriptor table accurately puts in
 * the double fault stack. If this failed that means the double fault was
 * called from a stack overflow and the stack doesn't have room for the 
 * interrupt stack frame and the GDT didn't correctly put in the stack
 * frame
 */
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use cascade::{
    test_panic_handler,
    serial_print,
    serial_println,
    cpu::{
        exit_qemu,
        QemuExitCode,
    },
};
use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault)
                .set_stack_index(cascade::gdt::DOUBLE_FAULT_IST_INDEX);
            }
        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("\nstack_overflow::stack_overflow...\t");
    cascade::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

fn init_test_idt(){
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();
    volatile::Volatile::new(0).read(); //prevents the compiler from optimizing the stack frame away
}

#[panic_handler]
fn stack_panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

extern "x86-interrupt" fn test_double_fault(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}


