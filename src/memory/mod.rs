mod frame_allocator;
pub mod paging;
pub mod memory_set;
use buddy_system_allocator::LockedHeap;
use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use crate::consts::*;

use riscv::addr::{
    Frame
};

use memory_set::{
    attr::MemoryAttr,
    handler::Linear,
    MemorySet
};
pub fn kernel_remap(){
    let mut memory_set = MemorySet::new();
    extern "C" {
        fn bootstack();
        fn bootstacktop();
    }
    memory_set.push(
        bootstack as usize, 
        bootstacktop as usize, 
        MemoryAttr::new(), 
        Linear::new(PHYSICAL_MEMORY_OFFSET)
    );
    unsafe{
        memory_set.activate();
    }
}
pub fn init(l: usize, r:usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    kernel_remap();
    println!("++++ setup memory!    ++++");
}

pub fn alloc_frame() -> Option<Frame> {
    // ppn, physical page number, will return the `Frame` struct by the
    // frame's number (frame_number << 12 == address)
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    println!("++++ setup kernel heap ++++");
}

pub fn access_pa_via_va(pa: usize) -> usize{
    pa + PHYSICAL_MEMORY_OFFSET
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}