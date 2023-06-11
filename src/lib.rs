#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[cfg(test)]
use bootloader::{entry_point,BootInfo};
#[cfg(test)]
entry_point!(test_kernel_main);

pub mod serial;
pub mod vga_buff;
pub mod interrupt;
pub mod gdt;
pub mod memory;

/// Initialize the GDT, Interrupts and the IDT
pub fn init() {
    gdt::init();
    unsafe { interrupt::PICS.lock().initialize() };
    interrupt::init_idt();
    x86_64::instructions::interrupts::enable();
}

/// Trait used for testing every function
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    /// Prints the test's name when running test case
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Runs every test case (defined in the global macro test_runner)
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Panics if a test fails
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Represents the VM's status code given to QEMU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits QEMU
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// The kernels enter a loop where it's stopped until receiving an interrupts. Prevents active event listening with infinite loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
fn test_kernel_main (_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}