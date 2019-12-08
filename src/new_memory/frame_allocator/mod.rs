//use crate::memory::buddy_allocator::{BuddyAllocator, log2_down};
use super::buddy_allocator::{BuddyAllocator, log2_down};
use super::Frame;

use lazy_static::*;
use spin::Mutex;
//use crate::riscv::addr::{Frame, PhysAddr};
use crate::consts;

// 物理页帧分配器
lazy_static! {
    pub static ref BUDDY_ALLOCATOR: Mutex<BuddyAllocator>
        = Mutex::new(BuddyAllocator::new());
}

pub fn init(start: usize, length: usize) {
    // 以page为单位进行管理和分配

    BUDDY_ALLOCATOR.lock()
        .init(log2_down((start + length - consts::MEMORY_OFFSET) / consts::PAGE_SIZE) as u8);
    // here we devide PAGE_SIZE because memory are considered as pages
//    BUDDY_ALLOCATOR.lock()
//        .init(log2_down((consts::MEMORY_END - consts::MEMORY_OFFSET) / consts::PAGE_SIZE) as u8);

    alloc_frames((start - consts::MEMORY_OFFSET) / consts::PAGE_SIZE);
    println!("++++init frame allocator succeed!++++");
}

pub fn alloc_frame() -> Option<Frame> {
    alloc_frames(1)
}
// buddy_allocator::alloc 返回的是内存块编号，类型为 Option<usize> ，
// 所以需要将其转换为物理地址，然后通过 Frame::of_addr 转换为物理帧。
// 同理，在释放内存时需要进行类似的操作。
pub fn alloc_frames(size: usize) -> Option<Frame> {
    let ret = BUDDY_ALLOCATOR
        .lock()
        .alloc(size)
        .map(|id| id * consts::PAGE_SIZE + consts::MEMORY_OFFSET); // frame # to phy addr
    ret.map(|addr| Frame::containing_address(addr))
}

pub fn dealloc_frame(target: Frame) {
    dealloc_frames(target, 1);
}

pub fn dealloc_frames(target: Frame, size: usize) {
    BUDDY_ALLOCATOR
        .lock()
        .dealloc(target.start_address() / consts::PAGE_SIZE - consts::MEMORY_OFFSET / consts::PAGE_SIZE, size);
}

pub fn test() {
    let frame1: Frame = alloc_frame().expect("failed to alloc frame");
    println!("test frame_allocator: {:#x}", frame1.start_address());
    let frame2: Frame = alloc_frames(2).expect("failed to alloc frame");
    println!("test frame_allocator: {:#x}", frame2.start_address());
    let frame3: Frame = alloc_frame().expect("failed to alloc frame");
    println!("test frame_allocator: {:#x}", frame3.start_address());
    dealloc_frame(frame1);
    dealloc_frames(frame2, 2);
    dealloc_frame(frame3);
}
