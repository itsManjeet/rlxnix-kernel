#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use rlxos::{
    serial_print,
    exit_qemu,
    QemuExitCode,
    serial_println,
};
use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};


lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rlxos::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    rlxos::gdt::initialize();
    init_test_idt();

    stack_overflow();
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rlxos::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: u64,) -> ! {
    serial_println!("[pass]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}