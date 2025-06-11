#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// Main entry point of program -> will always be _start regardless of host os
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {

    }
}
