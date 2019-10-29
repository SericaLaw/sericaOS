extern crate alloc;
use crate::context::Context;
use alloc::alloc::{alloc, dealloc, Layout};
use alloc::boxed::Box;
// TODO: replace satp here
use riscv::register::satp;
use crate::consts::STACK_SIZE;
use crate::process::{Tid, ExitCode};

pub struct Thread {
    pub context: Context, // 线程相关的上下文
    pub kstack: KernelStack, // 线程对应的内核栈
}

// 其实内核栈保存了该内核线程的各种数据以及上下文内容，本质上它就是一片固定大小的内存空间，因此我们只需要在 KernelStack 中记录栈的起始地址。
pub struct KernelStack(usize);

#[derive(Clone)]
pub enum Status {
    Ready,
    Running(Tid),
    Sleeping,
    Exited(ExitCode),
}

impl Thread {
    // 在操作系统中，有一个特殊的线程，其名字为为 idle 。
    // 其作用是初始化一些信息，并且在没有其他线程需要运行的时候运行他。
    pub fn new_idle() -> Box<Thread> {
        unsafe {
            Box::new(Thread {
                context: Context::null(),
                kstack: KernelStack::new(),
            })
        }
    }

    // 内核线程
    pub fn new_kernel(entry: extern "C" fn(usize) -> !, arg: usize) -> Box<Thread> {
        unsafe {
            let kstack_ = KernelStack::new();
            Box::new(Thread {
                context: Context::new_kernel_thread(entry, arg, kstack_.top(), satp::read().bits()),
                kstack: kstack_,
            })
        }
    }

    // 创建好线程之后，则需要有办法能够在多个线程中相互切换。
    // 切换的过程需要两步：
    // 保存当前寄存器状态。
    // 加载另一线程的寄存器状态。
    pub fn switch_to(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }
}

/// 为了实现简单，栈空间就直接从内核堆中分配了。
/// 我们需要在它的构造函数（new）中分配内存，并在析构函数（Drop）中回收内存。
/// 具体而言，我们使用 rust 的 alloc API 实现内存分配和回收
/// 即使用全局分配其linked list allocator
impl KernelStack {
    pub fn new() -> KernelStack {
        let bottom =
            unsafe {
                alloc(Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap())
            } as usize;
        KernelStack(bottom)
    }

    pub fn top(&self) -> usize {
        self.0 + STACK_SIZE
    }
}

/// https://doc.rust-lang.org/book/ch15-03-drop.html
impl Drop for KernelStack {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.0 as _,
                Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap()
            );
        }
    }
}