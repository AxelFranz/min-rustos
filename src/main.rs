#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";


#[panic_handler] // On crée notre panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start(){
    
    let vga_buffer = 0xb8000 as *mut u8; // Adresse du VGA Buffer

    for (i,&byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // On print la lettre
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // On set la couleur
        }
    }

    loop{}
}

