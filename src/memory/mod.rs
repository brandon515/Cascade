pub mod allocator;

use x86_64::{
    PhysAddr,
    structures::{
        paging::{
            PageTable,
            OffsetPageTable,
            PhysFrame,
            FrameAllocator,
            Size4KiB,
        },
    },
    VirtAddr,
    registers::control::Cr3,
};


use bootloader::bootinfo::{
    MemoryMap,
    MemoryRegionType,
};

use alloc::{
    boxed::Box,
    vec::Vec,
};


pub struct HeapFrameAllocator{
    memory_iter: Box<dyn Iterator<Item = PhysFrame>>,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.memory_iter.next()
    }
}

impl HeapFrameAllocator{
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        //get all usable regions of memory
        let regions = memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        //map each regions to it's address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        //get an address 4kb apart 
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        //now the containing_address function gets the start address of each frame, even if the
        //address is in the middle of the frame
        let frame_list: Vec<PhysFrame> = frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr))).collect();
        HeapFrameAllocator{
            memory_iter: Box::new(frame_list.into_iter()),
        }
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    //TODO implement a linked list or vec to collect into and save an iterator to once we have heap
    //memory
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        //get all usable regions of memory
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        //map each regions to it's address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        //get an address 4kb apart 
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        //now the containing_address function gets the start address of each frame, even if the
        //address is in the middle of the frame
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))

    }
}

pub unsafe fn init(offset: VirtAddr) -> OffsetPageTable<'static>{
    let level_4_table = active_level_4_table(offset);
    OffsetPageTable::new(level_4_table, offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    let (level_4_table_frame, _) = Cr3::read(); //get the physical address from the bootloader

    let physical_address = level_4_table_frame.start_address(); //the start of the physical address
    // this is the offset where the bootloader put the phyical memory mapped to a page table
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_pointer: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *page_table_pointer
}

