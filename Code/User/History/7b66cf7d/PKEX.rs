#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mini_kernel::{QemuExitCode, exit_qemu, serial_println};
use mini_kernel::serial_print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");

    exit_qemu(QemuExitCode::Success);

    loop {

    }
}

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");

    assert_eq!(0, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {

    }
}