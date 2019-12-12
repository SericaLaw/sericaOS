use super::{Page, ENTRY_COUNT, PAGE_SIZE, Frame, FrameAllocator};
use super::entry::*;
use super::table::{self, Table, Level2};

use core::ptr::Unique;
pub struct Mapper {
    p2: Unique<Table<Level2>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper {
            p2: Unique::new_unchecked(table::P2),
        }
    }

    pub fn p2(&self) -> &Table<Level2> {
        unsafe { self.p2.as_ref() }
    }

    pub fn p2_mut(&mut self) -> &mut Table<Level2> {
        unsafe { self.p2.as_mut() }
    }

    /// Translates a virtual to the corresponding physical address.
    /// Returns `None` if the address is not mapped.
    pub fn translate(&self, virtual_address: usize) -> Option<usize> {
        let offset = virtual_address & (PAGE_SIZE - 1);
        println!("translate virt addr 0x{:x?}", virtual_address);
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        println!("translate page {:x?}", page);
        let p1 = self.p2().next_table(page.p2_index());


        let res = p1.and_then(|p1| p1[page.p1_index()].pointed_frame());

        println!("translate page: {:x?} => {:x?}", page, res);
        res
    }


    /// Maps the page to the frame with the provided flags.
    /// The `VALID` flag is added by default. Needs a
    /// `FrameAllocator` as it might need to create new page tables.
    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: u32,
                     allocator: &mut A)
        where A: FrameAllocator
    {
        let mut p1 = self.p2_mut().next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_invalid());
        p1[page.p1_index()].set(frame, flags | EntryBits::Valid.val());
    }

    /// Maps the page to some free frame with the provided flags.
    /// The free frame is allocated from the given `FrameAllocator`.
    pub fn map<A>(&mut self, page: Page, flags: u32, allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    /// Identity map the the given frame with the provided flags.
    /// The `FrameAllocator` is used to create new page tables if needed.
    pub fn identity_map<A>(&mut self,
                           frame: Frame,
                           flags: u32,
                           allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    pub fn linear_map<A>(&mut self,
                         frame: Frame,
                         offset: u32,
                         flags: u32,
                         allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::containing_address(frame.start_address() + offset as usize);
        self.map_to(page, frame, flags, allocator)
    }

    /// Unmaps the given page and adds all freed frames to the given
    /// `FrameAllocator`.
    pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        use crate::riscv::instructions;

        println!("active table unmap page: {:x?}", page);

        assert!(self.translate(page.start_address()).is_some());

        let mut del_p1 = true;

        let p1 = self.p2_mut()
            .next_table_mut(page.p2_index())
            .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        println!("{:x?} is valid, start to unmap, the pointed frame is {:x?}", page, frame);
        p1[page.p1_index()].set_entry(0);
        for i in 0..ENTRY_COUNT {
            if p1[i].is_valid() {
                del_p1 = false;
                break;
            }
        }
        instructions::flush_tlb();
        // TODO free p(1,2,3) table if empty
//        allocator.deallocate_frame(frame);

        if del_p1 {
            allocator.deallocate_frame(self.p2()[page.p2_index()].pointed_frame().unwrap());
        }

        self.p2_mut()[page.p2_index()].set_entry(0);
        assert!(self.translate(page.start_address()).is_none());
    }
}