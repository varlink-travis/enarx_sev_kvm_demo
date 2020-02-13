#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

use core::panic::PanicInfo;
use kernel::arch::OffsetPageTable;
use kernel::memory::BootInfoFrameAllocator;
use kernel::{entry_point, exit_hypervisor, serial_println, HyperVisorExitCode};
use vmbootspec::BootInfo;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    fn inner(_mapper: &mut OffsetPageTable, _frame_allocator: &mut BootInfoFrameAllocator) -> ! // trigger a stack overflow
    {
        serial_println!("check stack_overflow");
        init_test_idt();
        stack_overflow();

        panic!("Execution continued after stack overflow");
    }

    kernel::arch::init(boot_info, inner)
}

#[inline(always)]
pub fn read_rsp() -> u64 {
    let val: u64;
    unsafe { asm!("mov $0, rsp" : "=r"(val) ::: "intel", "volatile") }
    val
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    //serial_println!("stackpointer: {:#X}", read_rsp());
    stack_overflow(); // for each recursion, the return address is pushed
}

pub static mut TEST_IDT: Option<InterruptDescriptorTable> = None;

pub fn init_test_idt() {
    unsafe {
        TEST_IDT.replace({
            let mut idt = InterruptDescriptorTable::new();
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(kernel::arch::x86_64::gdt::DOUBLE_FAULT_IST_INDEX);
            idt
        });
        TEST_IDT.as_ref().unwrap().load();
    }
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    //serial_println!("[ok]");
    exit_hypervisor(HyperVisorExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
