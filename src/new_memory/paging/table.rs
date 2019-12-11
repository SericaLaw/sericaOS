use crate::new_memory::paging::entry::*;
use crate::new_memory::paging::ENTRY_COUNT;
use crate::new_memory::{Frame, FrameAllocator};

use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

pub struct Table<L: TableLevel> {
    pub entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> core::fmt::Debug for Table<L> where L: TableLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:x?})", &self.entries[0] as *const _ as i32)
    }
}

impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

// TODO: replace P2 with ROOT_PAGE_TABLE: ... = consts:...
pub const P2: *mut Table<Level2> = 0xffff_e000 as *mut _;

impl<L> Table<L> where L: TableLevel {
    pub fn init(&mut self) {
        for i in 0..ENTRY_COUNT {
            self[i].set_entry(0);
        }
    }
}

/// sv32
impl<L> Table<L> where L: HierarchicalLevel {
    // get virtual address of next table which table[index] points to
    fn next_table_address(&self, index: usize) -> Option<usize> {
//        println!("entry for next table: 0x{:x?}", self.entries[index]);
        if self[index].is_valid() {
            let table_address = self as *const _ as usize;
            Some(((table_address >> 12 + 1) << 10 | index) << 12)
        }
        else {
            None
        }
    }

    /// Note the additional lifetime parameters, which are identical for input and output references.
    /// That's exactly what we want. It ensures that we can't modify tables as long as we have references to lower tables.
    /// For example, it would be very bad if we could unmap a P3 table if we still write to one of its P2 tables.
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe {
                println!("next table address: 0x{:x?}", address);
                &*(address as *const _)
            })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe {
                println!("next table mut address: 0x{:x?}", address);
                &mut *(address as *mut _)
            })
    }

    // It should return the next table if it exists, or create a new one.
    pub fn next_table_create<A>(&mut self,
                                index: usize, // page index
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            // TODO: zalloc
            let frame = allocator.allocate_frame().expect("no frames available");
            // set RWX temporarily, by indicating the entry points to a leaf, the new frame can be accessed by virtual address
            self.entries[index].set(frame, EntryBits::Valid.val() | EntryBits::ReadWrite.val());

            self.next_table_mut(index).unwrap().init();
            // reset to indicate it is a branch rather than a leaf
            self.entries[index].set(frame, EntryBits::Valid.val());
            println!("create next table:\n\tentry 0x{:x?}({:x?}) maps to {:x?}", index, self[index], frame);
        }
        self.next_table_mut(index).unwrap()
    }
}

pub trait TableLevel {}

pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}