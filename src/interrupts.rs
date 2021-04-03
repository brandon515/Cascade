use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};
use crate::println;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32; //CPU has 32 execptions, so the first index that are free is 32 since the exceptions are 0-31
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // PIC 1 has 8 interrupts


//we use lazy static here because the IDT needs to have a static lifetime because it's
//used the entire time the OS is running. We could use a Box<> but we don't have a heap
//yet.
lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) } );

pub fn init_idt(){
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

//error_code is always 0 for double faults so it's useless here
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    println!("EXECPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}

#[test_case]
fn test_breakpoint_exception(){
    x86_64::instructions::interrupts::int3();
}
