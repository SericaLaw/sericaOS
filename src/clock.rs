use crate::riscv::sbi::set_timer;

use riscv::register::sie; // TODO: figure out how it works
use crate::riscv::register::{ time, timeh };

pub static mut TICK: usize = 0;
static TIMEBASE: u64 = 100000;

pub fn init() {
    println!("++++setup timer !++++");
    unsafe {
        TICK = 0;
        // sie::set_stimer 通过将 mie 寄存器的 STIE 位（第 5 位）设为 1 开启了内核态的时钟中断。
        sie::set_stimer();
//        let mut sie: usize;
        //        asm!("csrci mstatus, 0b10"::::"volatile");
//        asm!("csrr x10, sie"
//            :"={x10}"(sie)
//            :::"volatile");
//        println!("sie: {:#x?}", sie);
//        sie |= 0b10000;
//        println!("sie: {:#x?}", sie);
        //        asm!("csrci sstatus, 0b10"::::"volatile");
//        asm!("csrw sie, x10"
//                    ::"{x10}"(sie)
//                    ::"volatile");
//        asm!("csrr x10, sie"
//            :"={x10}"(sie)
//            :::"volatile");
//        println!("sie: {:#x?}", sie);
//        let set: usize = 1 << 5;
//        asm!("csrci sie, 0b10000"::::"volatile");
    }
    clock_set_next_event();
    println!("++++setup timer !++++");
}

// 设置下一次时钟中断触发的时间。riscv 不支持直接设置时钟中断的间隔，只能在每次触发时钟中断的时候，设置下一次时钟中断的时间。
// TIMEBASE 是时间间隔，其数值一般约为 cpu 频率的 1% ，防止时钟中断占用过多的 cpu 资源。
pub fn clock_set_next_event() {
    println!("++++setup timer !++++");
    set_timer(get_cycle() + TIMEBASE);
}

// 获取当前时间，当前时间加上 TIMEBASE 为下一次中断产生的时间，通过 set_timer 设置。
fn get_cycle() -> u64 {
    // cpu 中有一个专门用于储存时间的 64 位寄存器。由于 system call 的返回值存放于 32 位的 x10 通用寄存器，所以需要分别读取时间的前 32 位和后 32 位：
    // hi 是时间的高 32 位，lo 是时间的低 32 位。注意到这里并没有之间拼接 hi 和 lo 然后将其返回，而是多了一步 if hi == tmp 判断。
    // 这是由于在执行完 let lo = time::read() 后，当前时间会改变。尽管时间的前 32 位改变的概率很小，但是仍然需要进行一次判断。
    loop {
        let mut hi: usize;
        let mut lo: usize;
        let mut tmp: usize;
        unsafe {
            hi = timeh::read();
            lo = time::read();
            tmp = timeh::read();
        }
        if hi == tmp {
            println!("time {} {}", hi, lo);
            return ((hi as u64) << 32) | (lo as u64);
        }
    }
}