use crate::new_memory::{Frame, PAGE_ORDER};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum EntryBits {
    None = 0,
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Access = 1 << 6,
    Dirty = 1 << 7,

    // Convenience combinations
    ReadWrite = 1 << 1 | 1 << 2,
    ReadExecute = 1 << 1 | 1 << 3,
    ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

    // User Convenience Combinations
    UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
    UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
    UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

// Helper functions to convert the enumeration
// into an i64, which is what our page table
// entries will be.
impl EntryBits {
    pub fn val(self) -> u32 {
        self as u32
    }
}

// A single entry. We're using an i64 so that
// this will sign-extend rather than zero-extend
// since RISC-V requires that the reserved sections
// take on the most significant bit.

pub struct Entry {
    pub entry: u32,
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Entry (0x{:x}, 0b{:b})", self.entry >> 10, self.entry & 0xff)
    }
}

// The Entry structure describes one of the 512 entries per table, which is
// described in the RISC-V privileged spec Figure 4.18.
impl Entry {
    pub fn is_valid(&self) -> bool {
        self.get_entry() & EntryBits::Valid.val() != 0
    }

    // The first bit (bit index #0) is the V bit for
    // valid.
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    // A leaf has one or more RWX bits set
    pub fn is_leaf(&self) -> bool {
        self.get_entry() & 0xe != 0
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn set(&mut self, frame: Frame, flags: u32) {
        self.entry = (frame.start_address() as u32) >> 2 | flags;
    }

    pub fn set_entry(&mut self, entry: u32) {
        self.entry = entry;
    }

    pub fn get_entry(&self) -> u32 {
        self.entry
    }

    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.is_valid() {
            Some(Frame::containing_address(
                (self.get_entry() as usize >> 10) << PAGE_ORDER
            ))
        } else {
            None
        }
    }
}