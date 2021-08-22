use core::{
    ptr::null_mut,
    alloc::{
        GlobalAlloc,
        Layout,
    },
};

use x86_64::{
    structures::paging::{
        mapper::MapToError,
        FrameAllocator,
        Mapper,
        Page,
        PageTableFlags,
        Size4KiB,
    },
    VirtAddr,
};

use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 2024; //100 KiB
pub struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8{
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout){
        panic!("no memory should be allocated so this should never be called");
    }
}

#[global_allocator]
static A: LockedHeap = LockedHeap::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // get the range of virtual pages to map to physical memory
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start+HEAP_SIZE-1u64;// make sure we're in the page memory space by subtracting 1
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    //now go through every page and use the provided frame allocator to map it to a physical memory
    //space
    for page in page_range {
        //just gives us the next free physical memory block
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe {
        A.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
