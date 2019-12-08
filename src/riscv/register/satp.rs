pub unsafe fn write(root_table_addr: usize) {
    asm!("csrw satp, $0"
    :: "r"(root_table_addr)
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


