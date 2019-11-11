extern crate alloc;
use core::cell::UnsafeCell; // UnsafeCell 内的元素不严格区分 immutable 和 mutable
use alloc::boxed::*;
use crate::process::Tid;
use crate::process::structs::*;
use crate::process::thread_pool::ThreadPool;
use crate::interrupt::{disable_and_store, enable_and_wfi, restore};


// 由于创建的调度器是全局的，需要考虑一些安全问题和异步问题。为此需要对成员进行一些包装
pub struct ProcessorInner {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    current: Option<(Tid, Box<Thread>)>,
}

pub struct Processor {
    inner: UnsafeCell<Option<ProcessorInner>>,
}

impl Processor {
    pub const fn new() -> Processor {
        Processor {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn init(&self, idle: Box<Thread>, pool: Box<ThreadPool> ) {
        unsafe {
            // deref to get value contained, and then modify it
            *self.inner.get() = Some(ProcessorInner{
                pool,
                idle,
                current: None,
            });
        }
    }

    fn inner(&self) -> &mut ProcessorInner {
        unsafe { &mut *self.inner.get() }
            .as_mut()
            .expect("Processor is not initialized...")
    }

    pub fn add_thread(&self, thread: Box<Thread>) {
        self.inner().pool.add(thread);
    }

    // 这是整个调度过程最核心的函数，由 idle 线程调用。
    pub fn run(&self) -> !{
        let inner = self.inner();
        // 关闭中断，防止此时产生中断异常导致线程切换出错。
        disable_and_store();
        // 循环从线程池中寻找可调度线程
        loop {
            // 如果存在需要被调度的线程
            if let Some(thread) = inner.pool.acquire() {
                inner.current = Some(thread);
                // 切换至需要被调度的线程
                inner.idle.switch_to(&mut *inner.current.as_mut().unwrap().1);
                // 上一个线程已经结束或时间片用完，切换回 idle 线程
                let (tid, thread) = inner.current.take().unwrap();
                println!("thread {} ran just now", tid);
                // 将上一个线程放回线程池中
                inner.pool.retrieve(tid, thread);
            } else {
                // 开启中断并等待中断产生
                enable_and_wfi();
                // 关闭中断，从线程池中寻找可调度线程
                disable_and_store();
            }
        }
    }

    // 通知线程池和调度算法已经过了一个时钟周期，同时返回一个布尔值：是否需要进行线程切换。
    // 如果需要切换至其他线程，则先切换至 idle 线程，然后由 idle 进行调度（回到 Processer.run）
    pub fn tick(&self) {
        let inner = self.inner();
        if !inner.current.is_none() {
            if inner.pool.tick() {
                let flags = disable_and_store();
                inner
                    .current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut inner.idle);
                // 恢复原先的中断状态
                restore(flags);
            }
        }
    }

    // 当线程任务完成之后，就可以通过 Processor.exit 结束自己（结束当前线程）
    pub fn exit(&self, code: usize) -> ! {
        let inner = self.inner();
        let tid = inner.current.as_ref().unwrap().0;
        // 通知线程池该线程即将退出
        inner.pool.exit(tid, code);
        // 切换至 idle 线程，进入调度
        inner
            .current
            .as_mut()
            .unwrap()
            .1
            .switch_to(&mut inner.idle);
        loop {}
    }
}

// 一个实现了 Sync trait 的类型可以安全的在多个线程中拥有其值的引用。
unsafe impl Sync for Processor {}