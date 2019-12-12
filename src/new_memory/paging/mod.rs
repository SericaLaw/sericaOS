pub mod entry;
pub mod table;
pub mod temporary_page;
pub mod mapper;

const ENTRY_COUNT: usize = 1024;
use super::{PAGE_ORDER, PAGE_SIZE, Frame, FrameAllocator};
use self::temporary_page::TemporaryPage;
use self::mapper::Mapper;
use core::ops::{Deref, DerefMut};
use self::entry::*;
use crate::new_memory::print_entry;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    pub number: usize,
}

impl core::fmt::Debug for Page {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Page (0x{:x?})", self.number)
    }
}

impl Page {
    pub fn containing_address(address: usize) -> Page {
        Page { number: address >> PAGE_ORDER }
    }

    pub fn start_address(&self) -> usize {
        self.number << PAGE_ORDER
    }

    // return the different table indexes
    pub fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }
    pub fn p1_index(&self) -> usize {
        (self.number >> 0) & 0x3ff
    }

    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end,
        }
    }
}

pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    // temporary changes the recursive mapping and executes a given closure in the new context
    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage, // new
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        use crate::riscv::{instructions, register::satp};
        println!("\n========== with ==========");
        {
            let backup = Frame::containing_address(satp::root_table_paddr());


            // map temporary_page to current p2 table
            let tp2 = temporary_page.p2_index();
            let tp1 = temporary_page.p1_index();
            let p2_table = temporary_page.map_table_frame(backup.clone(), self);
            println!("backup: {:x?}\ntemp page map:", backup);
            print_entry(2, [tp2, 0, 0]);
            print_entry(1, [tp2, tp1, 0]);
            println!("... overwrite recursive mapping");

//            // overwrite recursive mapping
            self.p2_mut()[ENTRY_COUNT - 1].set(table.p2_frame.clone(), EntryBits::Valid.val());
            instructions::flush_tlb();
            println!("new table frame: {:x?}", table.p2_frame);
            print_entry(0, [1023, 1022, 1022]);
//            println!("overwrite recursive mapping: {:x?} => {:x?}", self.p2_mut()[ENTRY_COUNT - 2].get_entry(), self.p2_mut()[ENTRY_COUNT - 1].get_entry());
            // execute f in the new context
            f(self);
//            println!("closure done\n");
//            println!("510: {:x?}", self.p2_mut().next_table(1022).unwrap().entries[769].get_entry());
//
//            // restore recursive mapping to original p2 table
//            p2_table[ENTRY_COUNT - 1].set(backup, EntryBits::Valid.val());
//
//            instructions::flush_tlb();

            println!("==========with done==========\n")
        }
// TODO
//        temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        println!("==========switch==========");
        use crate::riscv::{instructions ,register::satp};
        let old_table = InactivePageTable {
            p2_frame: Frame::containing_address(
                satp::root_table_paddr()
            ),
        };

        unsafe {
            satp::set_root_table(satp::Mode::Sv32, 0, new_table.p2_frame.number);

            instructions::flush_tlb();
        }
        println!("==========switch done==========\n");
        old_table
    }
}

pub struct InactivePageTable {
    pub p2_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable {
        {

            println!("\n==========new inactive==========");
            let table = temporary_page.map_table_frame(frame.clone(),
                                                       active_table);
            // now we are able to zero the table
            table.init();
            // set up recursive mapping for the table
            table[ENTRY_COUNT - 2].set(frame.clone(), EntryBits::Valid.val() | EntryBits::ReadWrite.val());
            table[ENTRY_COUNT - 1].set(frame.clone(), EntryBits::Valid.val());
            assert_eq!(table[ENTRY_COUNT - 1].get_entry() >> 10, (frame.start_address() >> 12) as u32);
        }

        {

            println!("set recursive:");
            super::print_entry(0, [temporary_page.p2_index(), temporary_page.p1_index(), 1022]);
            super::print_entry(0, [temporary_page.p2_index(), temporary_page.p1_index(), 1023]);

        }
        temporary_page.unmap(active_table);
        super::print_entry(2, [temporary_page.p2_index(), 0, 0]);

        println!("==========new inactive done==========\n");
        InactivePageTable { p2_frame: frame }
    }
}
