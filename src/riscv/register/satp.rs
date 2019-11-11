pub unsafe fn write(addr: usize) {
    asm!("csrw satp, $0"
    :: "r"(addr)
    :: "volatile");
}

pub fn read() -> usize {
    let mut bits: usize;
    unsafe {
        asm!("csrr x10, satp"
            :"={x10}"(bits)
            :::"volatile");
    }
    bits
}