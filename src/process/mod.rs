mod structs;
mod scheduler;
mod thread_pool;
mod processor;

use structs::Thread;
use processor::Processor;
use scheduler::Scheduler;
use thread_pool::ThreadPool;

use alloc::boxed::Box;

pub type Tid = usize; // thread id
pub type ExitCode = usize;

static CPU: Processor = Processor::new();

pub fn tick() {
    CPU.tick();
}

pub fn init() {
    println!("+------ now to initialize process ------+");
    let scheduler = Scheduler::new(1);
    let thread_pool = ThreadPool::new(10, scheduler);
    CPU.init(Thread::new_idle(), Box::new(thread_pool));

}

pub fn run() {
    let thread0 = Thread::new_kernel(hello_thread, 0);
    CPU.add_thread(thread0);
    let thread1 = Thread::new_kernel(hello_thread, 1);
    CPU.add_thread(thread1);
    let thread2 = Thread::new_kernel(hello_thread, 2);
    CPU.add_thread(thread2);
    let thread3 = Thread::new_kernel(hello_thread, 3);
    CPU.add_thread(thread3);
    let thread4 = Thread::new_kernel(hello_thread, 4);
    CPU.add_thread(thread4);
    CPU.run();
}
#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("hello thread");
    println!("arg is {}", arg);
    for i in 0..2 {
        println!("{}{}{}{}{}{}{}{}", arg, arg, arg, arg, arg, arg, arg, arg);
        for j in 0..1000 {
        }
    }
    println!("end of thread {}", arg);
    CPU.exit(0)
}

pub fn exit(code: usize) {
    CPU.exit(code);
}

extern "C" {
    fn _user_img_start();
    fn _user_img_end();
}