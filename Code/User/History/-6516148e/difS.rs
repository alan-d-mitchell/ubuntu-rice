#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

// Main entry point of program -> will always be _start regardless of host os
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {

    }
}
