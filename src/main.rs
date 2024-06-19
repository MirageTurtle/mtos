#![no_std]
#![no_main]  // I need to overwrite the entry point

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // A panic handler function should never, so we use the never type `!` to indicate that
    loop {}
}

#[no_mangle]  // don't mangle the name of _start
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function named `_start` by default
    loop {}
}

