#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// linux
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}


///windows
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}


