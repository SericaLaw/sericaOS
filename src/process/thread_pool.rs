// in process/thread_pool.rs
extern crate alloc;
use crate::process::scheduler::Scheduler;
use crate::process::structs::{Status, Thread};
use alloc::{vec::Vec, boxed::*};
use crate::process::Tid;

struct ThreadInfo {
    status: Status,
    present: bool,
    thread: Option<Box<Thread>>,
}

pub struct ThreadPool {
    threads: Vec<Option<ThreadInfo>>, // 线程信号量的向量
    scheduler: Box<Scheduler>, // 调度算法
}

impl ThreadPool {
    // 构造函数规定了线程的最大数量 size 和调度算法。
    // 由于线程数组是已经创建好的，但是默认内容为 None ，
    // 所以在添加线程的时候只需要将从 Vec 中找到一个未使用的位置，
    // 把新线程的信息传递过去就可以了。 同时，不要忘记为调度算法传入线程 id 。
    pub fn new(size: usize, scheduler: Scheduler) -> ThreadPool {
        ThreadPool {
            threads: {
                let mut th = Vec::new();
                th.resize_with(size, Default::default);
                th
            },
            scheduler: Box::new(scheduler),
        }
    }

    fn alloc_tid(&self) -> Tid {
        for (i, info) in self.threads.iter().enumerate() {
            if info.is_none() {
                return i;
            }
        }
        panic!("alloc tid failed !");
    }

    pub fn add(&mut self, _thread: Box<Thread>) {
        let tid = self.alloc_tid();
        self.threads[tid] = Some(ThreadInfo{
            status: Status::Ready,
            present: true,
            thread: Some(_thread),
        });
        self.scheduler.push(tid);
        println!("tid to alloc: {}", tid);
    }

    pub fn acquire(&mut self) -> Option<(Tid, Box<Thread>)> {
        if let Some(tid) = self.scheduler.pop() {
            let mut thread_info = self.threads[tid].as_mut().expect("thread not exist !");
            thread_info.status = Status::Running(tid);
            return Some((tid, thread_info.thread.take().expect("thread not exist ")));
        } else {
            return None;
        }
    }

    pub fn retrieve(&mut self, tid: Tid, thread: Box<Thread> ) {
        let mut thread_info = self.threads[tid].as_mut().expect("thread not exist !");
        if thread_info.present {
            thread_info.thread = Some(thread);
            thread_info.status = Status::Ready;
            self.scheduler.push(tid);
        }
    }

    pub fn tick(&mut self) -> bool {
        // 通知调度算法时钟周期加一，询问是否需要调度
        self.scheduler.tick()
    }

    pub fn exit(&mut self, tid: Tid, code: usize) {
        self.threads[tid] = Some(ThreadInfo{
            status: Status::Ready,
            present: false,
            thread: None,
        });
        self.scheduler.exit(tid);
        println!("exit code: {}", code);
    }
}