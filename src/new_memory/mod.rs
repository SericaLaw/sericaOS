pub mod linked_list_allocator;
pub mod buddy_allocator;
pub mod frame_allocator;
pub mod paging;

use crate::consts;
use crate::HEAP_ALLOCATOR;
use crate::riscv::register::sstatus;
use crate::new_memory::paging::{Page, ActivePageTable};
use crate::new_memory::paging::entry::{EntryBits, Entry};
use core::ptr::null_mut;


//use crate::riscv::addr::{PhysAddr, VirtAddr};

const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}

impl core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Frame (0x{:x?})", self.number)
    }
}

impl Frame {
    pub fn containing_address(addr: usize) -> Frame {
        Frame { number: addr >> PAGE_ORDER }
    }

//    #[inline(always)]
//    pub fn of_ppn(ppn: usize) -> Self {
//        Frame(PhysAddr::new(ppn << 12))
//    }

    pub fn start_address(&self) -> usize {
        self.number << PAGE_ORDER
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
    fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0x3ff
    }

//    pub fn number(&self) -> usize { self.0.page_number() }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

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



//    frame_allocator::test();
    let mut allocator = AreaFrameAllocator;
    test_paging(&mut allocator);
    print_os_layout();
    remap_kernel(&mut allocator);

    println!("OK!");
}

/// table_level: 2, 1, 0, where 0 represents a frame
/// index: [p2idx, p1idx, p0idx]
pub fn print_entry(table_level: usize, index: [usize; 3]) {
    let mut p_ret: *mut Entry = null_mut();
    if table_level == 2 {
        p_ret = (0xffff_e000 | (index[0] << 2)) as *mut _;
    }
    else if table_level == 1 {
        p_ret = ( 0xffc0_0000 | (index[0] << 12) | (index[1] << 2)) as *mut _;
    }
    else if table_level == 0 {
        p_ret = ((index[0] << 22) | (index[1] << 12) | (index[2] << 2)) as *mut _;
    }
    else {
        panic!();
    }
    unsafe {
        let ret = p_ret.as_ref();
        if ret.is_some() {
            println!("P{}[{}(0x{:x})] = {:x?}", table_level, index[2 - table_level], index[2 - table_level], ret.unwrap());
        }
    }
}

/// https://docs.rs/crate/linked_list_allocator/0.6.4
fn init_heap() {
    //    static变量 在内核sp所指的stack上开辟
    static mut HEAP: [u8; consts::KERNEL_HEAP_SIZE] = [0; consts::KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, consts::KERNEL_HEAP_SIZE);
    }
}

pub struct AreaFrameAllocator;
impl AreaFrameAllocator {
    pub fn new() -> AreaFrameAllocator {
        AreaFrameAllocator
    }
}
impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        frame_allocator::alloc_frame()
    }
    fn deallocate_frame(&mut self, frame: Frame) {
        frame_allocator::dealloc_frame(frame)
    }
}


pub fn remap_kernel<A>(allocator: &mut A)
                       -> ActivePageTable
    where A: FrameAllocator
{

    println!("remap kernel");
    use self::paging::{InactivePageTable ,temporary_page::TemporaryPage};
    let mut temporary_page = TemporaryPage::new(Page { number: 0x20000 },
                                                allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };
    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        println!("in closure: ", );
        let offset = consts::KERNEL_OFFSET as usize - consts::MEMORY_OFFSET as usize;
        // identity map the VGA text buffer
        let uart_frame = Frame::containing_address(0x1000_0000);
        mapper.identity_map(uart_frame, EntryBits::ReadWrite.val(), allocator);
        println!("\n\tidentity map uart ......\n");
        print_entry(0, [1023, uart_frame.p2_index(), uart_frame.p1_index()]);
//
//        print_os_layout();
//        println!("==========remap==========\n");
        // map the kernel sections
        let text_start = Frame::containing_address(stext as usize - offset);
        let text_end = Frame::containing_address(etext as usize - offset - 1);
        for frame in Frame::range_inclusive(text_start, text_end) {
            mapper.linear_map(frame, offset as u32, EntryBits::ReadExecute.val(), allocator);
        }

        let ptext_start = Page::containing_address(stext as usize);
        let ptext_end = Page::containing_address(etext as usize);

        println!("\n\tremap text: {:x?}=>{:x?}...{:x?}=>{:x?}\n", ptext_start, text_start, ptext_end, text_end);
        print_entry(0, [1023, ptext_start.p2_index(), ptext_start.p1_index()]);
        println!("\n\tremap data......\n");
        let data_start = Frame::containing_address(sdata as usize - offset);
        let data_end = Frame::containing_address(edata as usize - offset - 1);
        for frame in Frame::range_inclusive(data_start, data_end) {
            mapper.linear_map(frame, offset as u32, EntryBits::ReadWrite.val(), allocator);
        }
        println!("\n\tremap read only data......\n");
        let rodata_start = Frame::containing_address(srodata as usize - offset);
        let rodata_end = Frame::containing_address(erodata as usize - offset - 1);
        for frame in Frame::range_inclusive(rodata_start, rodata_end) {
            mapper.linear_map(frame, offset as u32, EntryBits::Read.val(), allocator);
        }

        println!("\n\tremap bss......\n");
        let bss_start = Frame::containing_address(sbss as usize - offset);
        let bss_end = Frame::containing_address(ebss as usize - offset - 1);
        for frame in Frame::range_inclusive(bss_start, bss_end) {
            mapper.linear_map(frame, offset as u32, EntryBits::ReadWrite.val(), allocator);
        }
        println!("\n\tremap boot......\n");
        let boot_start = Frame::containing_address(bootstack as usize - offset);
        let boot_end = Frame::containing_address(bootstacktop as usize - offset - 1);
        for frame in Frame::range_inclusive(boot_start, boot_end) {
            mapper.linear_map(frame, offset as u32, EntryBits::ReadWrite.val(), allocator);
        }
//
    });
//
    let old_table = active_table.switch(new_table);
    println!("NEW TABLE!!!");

    // turn the old p4 page into a guard page
    let old_p2_page = Page::containing_address(
        old_table.p2_frame.start_address()
    );
    active_table.unmap(old_p2_page, allocator);
    println!("guard page at {:#x}", old_p2_page.start_address());

    active_table
}

pub fn print_os_layout() {
    println!("\n========== OS MEM LAYOUT ==========");
    use crate::riscv::register::satp;
    println!("page table frame: {:x?}", Frame::containing_address(satp::root_table_paddr()));
    print_entry(2, [1022, 0, 0]);
    print_entry(2, [1023, 0, 0]);
    println!("text: 0x{:x}..0x{:x}", stext as usize, etext as usize);
    println!("rodata: 0x{:x}..0x{:x}", srodata as usize, erodata as usize);
    println!("data: 0x{:x}..0x{:x}", sdata as usize, edata as usize);
    println!("bootstack: 0x{:x}..0x{:x}", bootstack as usize, bootstacktop as usize);
    println!("bss: 0x{:x}..0x{:x}", sbss as usize, ebss as usize);

    print_entry(2, [769, 0, 0]);
    println!("\n========== OS MEM LAYOUT ==========\n");

}

pub fn test_paging<A>(allocator: &mut A) where A: FrameAllocator {

    let mut page_table = unsafe {ActivePageTable::new()};
    let addr = 42 * 1024 * 4096;
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:x?}, map to {:x?}",
             page_table.translate(addr),
             frame);

    page_table.map_to(page, frame, EntryBits::ReadWrite.val(), allocator);
    println!("Some = {:x?}", page_table.translate(addr));
    unsafe {
        print_entry(2, [42, 0, 0]);
        print_entry(1, [42, 0, 0]);
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