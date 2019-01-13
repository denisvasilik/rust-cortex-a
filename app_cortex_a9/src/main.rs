#![feature(lang_items)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

extern crate panic_abort;

use cortex_a::{asm, regs::*};
use cortex_m_semihosting::{hprintln};

global_asm!(include_str!("start.S"));

#[no_mangle]
pub fn entry_point() -> ! {
    hprintln!("Hello, world!").unwrap();
    loop {
        asm::nop();
        unsafe {
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
        }
        let _stack_pointer = SP.get();
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
    // Intentionally left blank!!!
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
    // Intentionally left blank!!!
}

#[no_mangle]
pub extern fn rust_begin_unwind() {
    // Intentionally left blank!!!
}

#[no_mangle]
pub extern fn _Unwind_Resume() {

}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {

}