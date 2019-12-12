#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(const_fn)] // enable const function 在编译时计算出结果
#![feature(naked_functions)]
#![feature(ptr_internals)]
#![feature(panic_info_message)]
#![feature(linkage)]

extern crate alloc;

#[macro_use]
pub mod io;

pub mod lang_item;
pub mod syscall;
pub mod linked_list_allocator;

use crate::linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();