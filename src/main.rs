#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// TODO: write the comment
//       Peterlits<peterlitszo@outlook.com>
#[no_mangle]
pub extern fn abort () {
    panic!("abort!");
}

// TODO: write the comment
//       Peterlits<peterlitszo@outlook.com>
#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop { }
}

// this function is the *entry point*, since the linker looks for a function
// named `_start` by default.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

