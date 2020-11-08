#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga;
/*
 * Panic Handler
 */
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


/*
 * Entry Point
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World {}", "!");
    
    loop {}
}