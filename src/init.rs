use crate::consts::*;

global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        // this function is provided by `src/boot/linker64.ld`, whose address
        // matches the end of this OS Kernel.
        fn end();
    }

    crate::interrupt::init();
    crate::timer::init();
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    loop {}
}
