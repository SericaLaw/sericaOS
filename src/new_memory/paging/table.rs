use crate::new_memory::paging::entry::*;
use crate::new_memory::paging::ENTRY_COUNT;

use core::ops::{Index, IndexMut};
pub struct Table {
    entries: [Entry; ENTRY_COUNT],
}

impl Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

pub const P2: *mut Table = 0xffff_f000 as *mut _;

impl Table {
    fn next_table_address(&self, index: usize) -> Option<usize> {
        if self[index].is_valid() && self[index].is_branch() {
            let table_address = self as *const _ as usize;
            Some(table_address << 10 | index << 12)
        }
        else {
            None
        }
    }
}