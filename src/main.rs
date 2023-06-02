#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use min_rustos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    min_rustos::init();
    x86_64::instructions::interrupts::int3();

    println!("Je marche encore");

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    println!("je suis pas là");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_rustos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

