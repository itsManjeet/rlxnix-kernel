#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rlxos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rlxos::println;
use core::panic::PanicInfo;


/*
 * Entry Point
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World {}", "!");
    #[cfg(test)]
    test_main();

    loop {}
}


/*
 * Panic Handler
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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