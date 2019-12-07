pub mod linked_list_allocator;
pub mod buddy_allocator;
pub mod frame_allocator;
pub mod paging;

use crate::riscv::addr::{PhysAddr, VirtAddr};

const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}


impl Frame {
    pub fn containing_address(addr: usize) -> Frame {
        Frame { number: addr >> PAGE_ORDER }
    }

//    #[inline(always)]
//    pub fn of_ppn(ppn: usize) -> Self {
//        Frame(PhysAddr::new(ppn << 12))
//    }

    pub fn start_address(&self) -> PhysAddr {
        PhysAddr::new(self.number << PAGE_ORDER)
    }

//    pub fn p2_index(&self) -> usize { self.0.p2_index() }

//    pub fn p1_index(&self) -> usize { self.0.p1_index() }

//    pub fn number(&self) -> usize { self.0.page_number() }
}
