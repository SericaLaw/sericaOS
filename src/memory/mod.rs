pub mod linked_list_allocator;
pub mod buddy_allocator;
pub mod frame_allocator;

use crate::consts;
use crate::HEAP_ALLOCATOR;
use crate::riscv::register::sstatus;

pub fn init() {
    unsafe {
        sstatus::set_sum();
    }
    init_heap();
    let memory_start = (end as usize) + consts::PAGE_SIZE;
    let memory_size = consts::MEMORY_END - memory_start;
    frame_allocator::init(memory_start, memory_size);
    frame_allocator::test();
}

/// https://docs.rs/crate/linked_list_allocator/0.6.4
fn init_heap() {
    static mut HEAP: [u8; consts::KERNEL_HEAP_SIZE] = [0; consts::KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, consts::KERNEL_HEAP_SIZE);
    }
}

// Symbols provided by linker script
extern "C" {
    fn end();
}
