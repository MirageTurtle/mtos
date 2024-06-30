#![no_std]
#![no_main] // I need to overwrite the entry point

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // A panic handler function should never
    // so we use the never type `!` to indicate that
    loop {}
}

#[no_mangle] // don't mangle the name of _start
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // since the linker looks for a function named `_start` by default

    // output "Hello, World!" to the VGA buffer
    // static HELLO: &[u8] = b"Hello, World!";
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
    //     }
    // }

    // use vga_buffer mod
    vga_buffer::print_something();
    loop {}
}
