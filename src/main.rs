#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

use core::panic::PanicInfo;

// TODO: write the comment
//       Peterlits<peterlitszo@outlook.com>
#[no_mangle]
extern "C" fn abort () -> ! {
    panic!("abort!");
}

// TODO: write the comment
//       Peterlits<peterlitszo@outlook.com>
#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop { }
}

global_asm!(include_str!("boot/entry64.asm"));

pub fn console_putchar(ch: u8) {
    let ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}

// this function is the *entry point*, since the linker looks for a function
// named `_start` by default.
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');
    loop {}
}

