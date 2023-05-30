#![no_std]
#![no_main]

mod vga_buff;

use core::panic::PanicInfo;

#[panic_handler] // On crée notre panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start(){
    
    println!("Coucou {}","Axel");
    loop{}
}

