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

// this is to get arround rust limitations of not implementing types that aren't created in this
// crate.
// Memory allocators need to be mutable but are static and can't but mutable without a mutex
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked{
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
// this only works when align is a power of 2
// example:
//      align = 0b0001000
//      addr = 0b01010111
//      align-1 = 0b0000111
//      !(align-1) = 0b1111000
//      addr+align = 0b01011111
//      addr+align-1 = 0b01011110
//      (addr+align-1) & !(align-1) = 0b01011000
//  which aligns it to align and the top size. That way we don't stop on memory that's already been
//  allocated. For pages alignment will be 4096 or 0x0100 or 0b01000000000000
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

// Allocator that puts a list of pages onto the heap, obviously it's only usable... with a heap
pub struct HeapFrameAllocator{
    memory_map: &'static MemoryMap,
    memory_iter: Box<dyn Iterator<Item = PhysFrame>>,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        match self.memory_iter.next(){
            Some(x) => Some(x),
            None => self.refresh_memory(),
        }
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
        let four = 4*1024;
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(four));
        //now the containing_address function gets the start address of each frame, even if the
        //address is in the middle of the frame
        let frame_list: Vec<PhysFrame> = frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr))).collect();
        HeapFrameAllocator{
            memory_map: memory_map,
            memory_iter: Box::new(frame_list.into_iter()),
        }
    }

    pub fn refresh_memory(&mut self) -> Option<PhysFrame>{
        //get all usable regions of memory
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        //map each regions to it's address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        //get an address 4kb apart 
        let four = 4*1024;
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(four));
        //now the containing_address function gets the start address of each frame, even if the
        //address is in the middle of the frame
        let frame_list: Vec<PhysFrame> = frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr))).collect();

        let mut frame_iter = Box::new(frame_list.into_iter());
        let ret = frame_iter.next();
        self.memory_iter = frame_iter;
        ret
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

