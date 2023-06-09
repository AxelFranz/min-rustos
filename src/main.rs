#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use min_rustos::{println, print};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    min_rustos::init();
    println!("Bienvenue");

    #[cfg(test)]
    test_main();

    min_rustos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    min_rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_rustos::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

