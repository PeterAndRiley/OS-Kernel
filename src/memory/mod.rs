mod frame_allocator;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use riscv::addr::{
    VirtAddr,
    PhysAddr,
    Page,
    Frame
};

pub fn init(l: usize, r:usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    println!("++++ setup memory!    ++++");
}
