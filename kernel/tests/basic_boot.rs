#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use kernel::{println, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start_main(boot_info: &'static mut vmsyscall::bootinfo::BootInfo) -> ! {
    kernel::arch::init_syscall(boot_info);
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}
