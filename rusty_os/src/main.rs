#![no_std]
#![no_main]

use core::panic::PanicInfo;

///called on panic
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// entry points
/// linux
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

