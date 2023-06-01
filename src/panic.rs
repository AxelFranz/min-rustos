
use core::panic::PanicInfo;
use crate::*;

#[cfg(test)]
#[panic_handler] // creating panic handler
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}",info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

#[cfg(not(test))]
#[panic_handler] // creating panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop{}
}

