use crate::riscv::register::scause::{ Trap, Interrupt, Exception };
use crate::riscv::register::{ sscratch, sstatus, stvec };
use crate::context::TrapFrame;
use crate::clock::{ TICK, clock_set_next_event };
use crate::process::tick;
//use core::intrinsics::type_id;

//use riscv::register::sscratch;
//在 RISCV 特权指令集手册中，描述了与中断处理相关的 CSR 寄存器：
//
//sscratch: 一个字的临时存储空间，一般用来辅助中断处理
//sstatus: 系统状态寄存器
//stvec: 中断跳转地址
//scause: 中断或异常的原因
//sepc: 发生中断时的位置 / PC
//
//与中断相关的指令：
//
//sret: S 态中断返回
//ecall: 向底层执行环境发出调用请求，用来进行系统调用
//ebreak: 触发断点异常
//
//stvec 中包含了 向量基址（BASE） 和 向量模式（MODE） ，其中 向量基址（BASE） 必须按照 4 字节对齐。
//
//RISCV 中有两种中断入口模式：
//
//直接模式（Driect） MODE = 0 ，触发任何 中断异常 时都把 PC 设置为 BASE
//向量模式（Vectored） MODE = 1 ，对第 i 种 中断 ，跳转到 BASE + i * 4；对所有 异常 ，仍跳转到 BASE
//为了实现简单，我们采用第一种模式，先进入统一的处理函数，之后再根据中断/异常种类进行不同处理。

global_asm!(include_str!("trap/trap.asm"));

#[inline(always)]
pub fn enable_and_wfi() {    // 使能中断并等待中断
    unsafe {
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}

#[inline(always)]
pub fn disable_and_store() -> usize {    // 禁用中断并返回当前中断状态
    let bits: usize;
    unsafe {
        asm!("csrci sstatus, 1 << 1" : "=r"(bits) ::: "volatile");
    }
    bits & (1 << 1)
}

#[inline(always)]
pub fn restore(flags: usize) {    // 根据 flag 设置中断
    unsafe {
        asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}


// 初始化中断向量
pub fn init() {
    println!("Initializing interrupt vector base...");
    extern {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0); // 给中断 asm 初始化
        // sie 寄存器控制了所有内核态的中断。
        // 需要将其 SSIE 位（第 2 位）设为 1 ，内核态才能接受软件中断。
        //为了能够正确响应内核态的时钟中断，需要将 sie 寄存器进行设置：
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }

}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    // 在 riscv 中，发生中断指令的 pc 被存入 sepc 。对于大部分情况，中断处理完成后还回到这个指令继续执行。
    // 但对于用户主动触发的异常（例如ebreak用于触发断点，ecall用于系统调用），中断处理函数需要调整 sepc 以跳过这条指令。
    // 如果不inc sepc 则会反复输出 trap! 。

    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimerInterrupt) => super_timer(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(tf),
        Trap::Exception(Exception::LoadPageFault) => page_fault(tf),
        Trap::Exception(Exception::StorePageFault) => page_fault(tf),
        _ => panic!("unexpected trap: {:x?}", tf.scause.cause()),
    }
    // 返回汇编代码继续执行sret
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn super_timer() {
    // 响应当前时钟中断的同时，手动设置下一个时钟中断
    clock_set_next_event();
    unsafe {
        TICK = TICK + 1;
        if TICK % 100 == 0 {
            println!("{} ticks!", TICK);
        }
    }
    tick();
}

fn page_fault(tf: &mut TrapFrame) {
    println!("{:?} @ {:#x}", tf.scause.cause(), tf.stval);
    panic!("page fault");
}

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

fn syscall(tf: &mut TrapFrame) {
    tf.sepc += 4;   // 主动跳过当前指令
    match tf.x[17] {
        SYS_WRITE => {
            print!("{}", tf.x[10] as u8 as char);
        },
        SYS_EXIT => {
            println!("exit!");
            use crate::process::exit;
            exit(tf.x[10]);
        },
        _ => {
            println!("unknown user syscall !");
        }
    };
}