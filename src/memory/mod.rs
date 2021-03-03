mod frame_allocator;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use riscv::addr::Frame;

use buddy_system_allocator::LockedHeap;
use crate::consts::KERNEL_HEAP_SIZE;

pub fn init(l: usize, r:usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
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

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}

pub fn access_pa_via_va(pa: usize) -> usize{
    pa + super::consts::PHYSICAL_MEMORY_OFFSET
}
