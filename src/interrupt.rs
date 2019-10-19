use crate::context::TrapFrame;
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

// 初始化中断向量
pub fn init() {
    println!("Initializing interrupt vector base...");
    extern {
        fn __alltraps();
    }
    unsafe {
//        sscratch::write(0);
        asm!("csrwi sscratch, 0"::::"volatile");

//        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        let base: usize = __alltraps as usize;
        asm!("csrw stvec, x10"
            ::"{x10}"(base)
            ::"volatile");
    }

}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    println!("trap");
    // 在 riscv 中，发生中断指令的 pc 被存入 sepc 。对于大部分情况，中断处理完成后还回到这个指令继续执行。
    // 但对于用户主动触发的异常（例如ebreak用于触发断点，ecall用于系统调用），中断处理函数需要调整 sepc 以跳过这条指令。
    // 如果不inc sepc 则会反复输出 trap! 。
    tf.increase_sepc();
}
