extern crate alloc;

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
    crate::memory::init_heap();
    test_heap();
    
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    loop {}
}

fn test_heap() {
    use alloc::vec::Vec;
    use alloc::boxed::Box;
    println!("test for heap...");
    let v = Box::new(5);
    assert!(*v == 5);
    core::mem::drop(v);
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert!(vec.len() == 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("test for heap passed");
}
