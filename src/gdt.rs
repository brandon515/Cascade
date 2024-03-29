use x86_64::{
    VirtAddr,
    structures::{
        tss::TaskStateSegment,
        gdt::{
            GlobalDescriptorTable,
            Descriptor,
            SegmentSelector,
        },
    },
    instructions::{
        segmentation::{
            CS,
            Segment,
        },
        tables::load_tss,
    },
};
use lazy_static::lazy_static;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; //this number doesn't actually matter, as long as it's 0-7

lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init(){
    GDT.0.load();
    unsafe{
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
