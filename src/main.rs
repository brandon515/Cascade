#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cascade::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate cascade;
extern crate alloc;
use bootloader::{
    BootInfo,
    entry_point,
};
use core::panic::PanicInfo;
use cascade::{
    memory::{
        self,
        allocator,
    },
    println,
};
use x86_64::{
    VirtAddr,
};

//entry point!
entry_point!(kernel_main); //this macro checks to make sure the entry point is the correct function signature

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    cascade::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut page_table_mapper = unsafe{ memory::init(physical_memory_offset) };
    let mut _frame_allocator = {
        let mut temp_allocator= unsafe {memory::BootInfoFrameAllocator::new(&boot_info.memory_map)};
        allocator::init_heap(&mut page_table_mapper, &mut temp_allocator)
            .expect("Heap init failed");
        unsafe {memory::HeapFrameAllocator::new(&boot_info.memory_map)}
    };

    println!("{:?}", &boot_info.memory_map);

    #[cfg(test)]
    test_main();

    println!("Hello World{}", "!");
    cascade::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cascade::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cascade::hlt_loop();
}
