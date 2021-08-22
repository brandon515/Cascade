#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cascade::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use cascade::{
    memory::{
        allocator::{
            self,
            HEAP_SIZE,
        },
        self,
        BootInfoFrameAllocator,
    },
};
use x86_64::VirtAddr;
use alloc::{
    boxed::Box,
    vec::Vec,
};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    cascade::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut page_table_mapper = unsafe{ memory::init(physical_memory_offset) };
    let mut temp_allocator= unsafe {memory::BootInfoFrameAllocator::new(&boot_info.memory_map)};
    allocator::init_heap(&mut page_table_mapper, &mut temp_allocator)
        .expect("Heap init failed");
    let mut frame_allocator = unsafe {memory::HeapFrameAllocator::new(&boot_info.memory_map)};

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cascade::test_panic_handler(info)
}

#[test_case]
fn heap_frame_allocator(){
}

#[test_case]
fn simple_allocation() {
    let heap_val1 = Box::new(23);
    let heap_val2 = Box::new(35);
    assert_eq!(*heap_val1, 23);
    assert_eq!(*heap_val2, 35);
}

#[test_case]
fn reallocations() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n-1)*n/2);
}

#[test_case]
fn deallocations() {
    for i in 0..HEAP_SIZE+10 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
