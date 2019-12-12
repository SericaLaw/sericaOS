pub const VERSION: &str = "0.3.0";

pub const KERNEL_HEAP_SIZE: usize = 0x0010_0000;
pub const MEMORY_OFFSET: usize = 0x8000_0000;
pub const MEMORY_END: usize = 0x8800_0000;
pub const PAGE_SIZE: usize = 4096;
pub const KERNEL_OFFSET: usize = 0xC000_0000;
pub const STACK_SIZE: usize = 0x8000;

pub const USER_STACK_OFFSET: usize = 0x80000000 - USER_STACK_SIZE;
pub const USER_STACK_SIZE: usize = 0x10000;