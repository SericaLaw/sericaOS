use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)] // 表示对这个结构体按 C 语言标准 进行内存布局
#[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32], // General registers
    pub sstatus: usize, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: usize, // Scause register: record the cause of exception/interrupt/trap
}

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}