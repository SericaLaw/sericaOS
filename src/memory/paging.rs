use crate::riscv::addr::Frame;
use crate::memory::frame_allocator::alloc_frame;
use crate::memory_set::MemoryAttr;

// 该结构体包含了根页表的物理地址，根页表的页目录项。
// 由于我们采用线性映射，所以我们还需要保存线性映射的 offset
pub struct InactivePageTable {
    root_table: Frame,
    PDEs: [Option<Frame>; 1024],
    offset: usize,
}

impl InactivePageTable {
    // 首先我们给根页表分配一个页面大小的物理内存，
    // 以后可以作为长度为 1024 的 u32 数组使用。每个 u32 就是一个页目录项。
    // PDEs 为页目录项指向的页表的物理地址，用 None 初始化。
    pub fn new(_offset: usize) -> InactivePageTable {
        if let Some(_root_table) = alloc_frame() {
            return InactivePageTable {
                root_table: _root_table, // page frame number
                PDEs: [None; 1024],
                offset: _offset,
            }
        } else {
            panic!("oom");
        }
    }
    // 地址虚实转换过程编写页面的重新映射
    fn pgtable_paddr(&mut self) -> usize {
        self.root_table.start_address().as_usize()
    }

    fn pgtable_vaddr(&mut self) -> usize {
        self.pgtable_paddr() + self.offset
    }

    // start, end are vaddr
    pub fn set(&mut self, start: usize, end: usize, attr: MemoryAttr) {
        unsafe {
            let mut vaddr = (start >> 12) << 12; // 4K 对齐
            let pg_table = &mut *(self.pgtable_vaddr() as *mut [u32; 1024]);
            while vaddr < end {
                // 1-1. 通过页目录和 VPN[1] 找到所需页目录项
                let PDX = get_pde_index(vaddr);
                let PDE = pg_table[PDX];
                // 1-2. 若不存在则创建
                if PDE == 0 {
                    self.PDEs[PDX] = alloc_frame();
                    let PDE_PPN = self.PDEs[PDX].unwrap().start_address().as_usize() >> 12;
                    pg_table[PDX] = (PDE_PPN << 10) as u32 | 0b1; // pointer to next level of page table
                }
                // 2. 页目录项包含了叶结点页表（简称页表）的起始地址，通过页目录项找到页表
                let pg_table_paddr = (pg_table[PDX] & (!0x3ff)) << 2;
                // 3. 通过页表和 VPN[0] 找到所需页表项
                // 4. 设置页表项包含的页面的起始物理地址和相关属性
                let pg_table_2 = &mut *((pg_table_paddr as usize + self.offset) as *mut [u32; 1024]);
                pg_table_2[get_pte_index(vaddr)] = ((vaddr - self.offset) >> 2) as u32 | attr.0;
                vaddr += (1 << 12);
            }
        }
    }

    unsafe fn set_root_table(root_table: usize) { // 设置satp
        asm!("csrw satp, $0" :: "r"(root_table) :: "volatile");
    }

    unsafe fn flush_tlb() {
        asm!("sfence.vma"::::"volatile");
    }

    // 将新页表的物理地址写入 satp 寄存器，达到切换页表的目的。
    // tlb 可以理解为页表的缓存，用以加快虚拟地址转换为物理地址的速度。
    // 所以在切换页表之后需要通过 flush_tlb 清空缓存。
    pub unsafe fn activate(&mut self) {
        Self::set_root_table((self.pgtable_paddr() >> 12) | (1 << 31));
        Self::flush_tlb();
    }
}

// 获取 VPN[1] 和 VPN[0]
fn get_pde_index(addr: usize) -> usize {
    addr >> 22
}

fn get_pte_index(addr: usize) -> usize {
    (addr >> 12) & 0x3ff
}