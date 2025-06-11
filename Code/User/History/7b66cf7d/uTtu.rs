#![no_std]
#![no_main]

use core::panic::PanicInfo;
use mini_kernel::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");

    exit_qemu(QemuExitCode::Success);

    loop {

    }
}

