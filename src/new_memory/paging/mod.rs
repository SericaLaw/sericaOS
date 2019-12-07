pub mod entry;
pub mod table;

const ENTRY_COUNT: usize = 1024;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,
}