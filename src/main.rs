#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use min_rustos::{println};
use x86_64::structures::paging::Translate;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main (boot_info: &'static BootInfo) -> ! {
    use min_rustos::memory;
    use x86_64::{VirtAddr};

    min_rustos::init();
    println!("Bienvenue");
    println!();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe {memory::init(phys_mem_offset)};

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }


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

