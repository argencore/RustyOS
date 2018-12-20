#![no_std]
#![no_main]

use core::panic::PanicInfo;

///called on panic
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// entry points
/// linux
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // location of vga buffer

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; //letter
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color
        }
    }

    loop {}
}

