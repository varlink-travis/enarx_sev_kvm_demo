#![no_std]
#![no_main]
#![warn(dead_code)]
#![cfg_attr(feature = "unstable", feature(custom_test_frameworks))]
#![cfg_attr(feature = "unstable", test_runner(kernel::test_runner))]
#![cfg_attr(feature = "unstable", reexport_test_harness_main = "test_main")]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use kernel::arch::OffsetPageTable;
use kernel::memory::BootInfoFrameAllocator;
use kernel::{entry_point, exit_hypervisor, println, HyperVisorExitCode};
use vmbootspec::BootInfo;

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    fn with_stack_protection(
        mapper: &mut OffsetPageTable,
        frame_allocator: &mut BootInfoFrameAllocator,
    ) -> ! {
        kernel::arch::exec_app(mapper, frame_allocator);
    }

    kernel::arch::init(boot_info, with_stack_protection)
}

#[cfg(test)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    fn inner(_mapper: &mut OffsetPageTable, _frame_allocator: &mut BootInfoFrameAllocator) -> ! {
        test_main();
        println!("It did not crash!");
        exit_hypervisor(HyperVisorExitCode::Success);
        kernel::hlt_loop()
    }

    kernel::arch::init(boot_info, inner)
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    exit_hypervisor(HyperVisorExitCode::Failed);
    kernel::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
