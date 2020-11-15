#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rlxos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use rlxos::println;
use core::panic::PanicInfo;
use bootloader::{
    BootInfo,
    entry_point,
};


/*
 * Entry Point
 */
entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rlxos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    use rlxos::allocator;

    println!("Welcome to rlxos {}","!");
    rlxos::initialize();

    let phys_addr = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::initialize(phys_addr)};
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::initialize(&boot_info.memory_map)};

    allocator::initialize(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
        
    let x = Box::new(41);

    #[cfg(test)]
    test_main();

    rlxos::hlt_loop();
}


/*
 * Panic Handler
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rlxos::hlt_loop();
}



// Test cased
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rlxos::test_panic_handler(info)
}

#[test_case]
fn trival_assertion() {
    assert_eq!(1, 1);
}