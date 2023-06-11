#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use min_rustos::{println, memory::BootInfoFrameAllocator};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

// Entry point of the kernel
#[no_mangle]
fn kernel_main (boot_info: &'static BootInfo) -> ! {
    use min_rustos::memory;
    use x86_64::{VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };


    min_rustos::init();
    println!("Welcome user !");
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

/// Panic handler during testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_rustos::test_panic_handler(info);
}