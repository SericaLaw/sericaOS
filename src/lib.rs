#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(const_fn)] // enable const function 在编译时计算出结果
#![feature(naked_functions)]
#![feature(ptr_internals)]
#![feature(panic_info_message)]

#[macro_use]
pub mod io;

pub mod interrupt;
pub mod context;
pub mod clock;

pub mod riscv;

pub mod memory;
pub mod memory_set;
pub mod new_memory;
pub mod consts;

pub mod process;
extern crate alloc;

pub mod device;

use crate::memory::linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();