// Removing the use of std rust library for creating a freestanding rust binary
// This binary doesn't uses any of the operating system functionalities
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Creating a panic handler, since we are not using the std library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 2. We use #[no_mangle] so the linker can find the symbol "_start"
//    By default, Rust mangles name of variables, functions, structs to prevent collisions (e.g. renames it to _ZN3foo3bar3h3E) which the linker won't understand.
//    "unsafe" is required in Rust 2024+ when modifying linker symbols.
// 3. We use extern "C" to tell the rust compiler to generate a binary that
//    invokes this function using the C calling convention.
//    This ensures the hardware/bootloader knows how to call this function.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    static HELLO: &[u8] = b"Hello World!";

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // isize: System specific address
            // 32-bit system: isize = 32-bit
            // 64-bit system: isize = 64-bit
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
