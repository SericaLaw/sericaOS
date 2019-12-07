pub mod linked_list_allocator;
pub mod buddy_allocator;
pub mod frame_allocator;
pub mod paging;


use crate::consts;
use crate::HEAP_ALLOCATOR;
use crate::riscv::register::sstatus;

pub fn init() {
    println!("init memory {}",  core::mem::size_of::<usize>());
    unsafe {
        sstatus::set_sum();
    }
    init_heap();
    // boot/linker.ld 为 end 赋值，这个是 kernel 的结束虚拟地址，此时由于尚未启用页表，虚实地址相等。
    let memory_start = (end as usize - consts::KERNEL_OFFSET + consts::MEMORY_OFFSET) + consts::PAGE_SIZE; //  在此之后是 device tree base ，我们为其留出 PAGE_SIZE 大小的空间存放
    let memory_size = consts::MEMORY_END - memory_start;


    frame_allocator::init(memory_start, memory_size);
    frame_allocator::test();

//    remap_kernel();

//    println!("OK!");
}

/// https://docs.rs/crate/linked_list_allocator/0.6.4
fn init_heap() {
//    static变量 在内核sp所指的stack上开辟
    static mut HEAP: [u8; consts::KERNEL_HEAP_SIZE] = [0; consts::KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, consts::KERNEL_HEAP_SIZE);
    }
}

// 建立页表（remap kernel）。这个过程分为以下几步：
//
// 获取需要重新映射的内存范围（虚拟地址）。
// 设置页面属性。
// 设置页表，将虚拟地址映射至目标物理地址。
// Symbols provided by linker script 首先我们需要获取需要重新映射内存的虚拟地址范围
// 这些函数赋值由 linker.ld 完成，这里将他们作为 usize 使用。

extern "C" {
    // text
    fn stext();
    fn etext();
    // data
    fn sdata();
    fn edata();
    // read only
    fn srodata();
    fn erodata();
    // bss
    fn sbss();
    fn ebss();
    // kernel
    fn start();
    fn end();
    // boot
    fn bootstack();
    fn bootstacktop();
}

fn remap_kernel() {
    println!("remaping");
    let offset = consts::KERNEL_OFFSET as usize - consts::MEMORY_OFFSET as usize;
    println!("offset: {:#x}",offset);
    use crate::memory::paging::{ InactivePageTable, MemoryAttr };
    let mut pg_table = InactivePageTable::new(offset);

//    println!("offset: {:#x}\n{:#?}",offset, pg_table.root_table);

    pg_table.set(stext as usize, etext as usize, MemoryAttr::new().set_readonly().set_execute());
    pg_table.set(sdata as usize, edata as usize, MemoryAttr::new().set_WR());
    pg_table.set(srodata as usize, erodata as usize, MemoryAttr::new().set_readonly());
    pg_table.set(sbss as usize, ebss as usize, MemoryAttr::new().set_WR());
    pg_table.set(bootstack as usize, bootstacktop as usize, MemoryAttr::new().set_WR());
//    pg_table.set(dtb, dtb + MAX_DTB_SIZE, MemoryAttr::new().set_WR());
    unsafe {
        pg_table.activate();
    }
}