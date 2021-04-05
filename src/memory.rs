use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
    registers::control::Cr3,
};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    let (level_4_table_frame, _) = Cr3::read(); //get the physical address from the bootloader

    let physical_address = level_4_table_frame.start_address(); //the start of the physical address
    // this is the offset where the bootloader put the phyical memory mapped to a page table
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_pointer: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *page_table_pointer
}
