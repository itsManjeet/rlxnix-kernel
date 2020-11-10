#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rlxos::test_runner)]
#![reexport_test_harness_main = "test_main"]

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
    use rlxos::memory::active_level_4_table;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;

    println!("Welcome to rlxos {}","!");
    rlxos::initialize();

   let phys_addr = VirtAddr::new(boot_info.physical_memory_offset);
   let l4_table = unsafe { active_level_4_table(phys_addr)};

   for (i, entry) in l4_table.iter().enumerate() {
       if !entry.is_unused() {
           println!("L4 entry {}: {:?}", i, entry);

           let phys = entry.frame().unwrap().start_address();
           let virt = phys.as_u64() + boot_info.physical_memory_offset;
           let ptr = VirtAddr::new(virt).as_mut_ptr();
           let l3_table: &PageTable = unsafe {&*ptr};

           for (i, entry) in l3_table.iter().enumerate() {
               if !entry.is_unused() {
                   println!("   L3 Entry {}: {:?}", i, entry);
               }
           }
       }
   }
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