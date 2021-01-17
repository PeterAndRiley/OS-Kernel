global_asm!(include_str!("boot/entry64.asm"));

use crate::interrupt::init;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::interrupt::init();
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
}
