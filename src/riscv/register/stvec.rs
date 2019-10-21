/// Trap mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits: usize = addr + mode as usize;
    asm!("csrw stvec, x10"
            ::"{x10}"(bits)
            ::"volatile");
}