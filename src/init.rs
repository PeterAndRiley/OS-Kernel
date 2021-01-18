global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::interrupt::init();
    crate::timer::init();
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    loop {}
}
