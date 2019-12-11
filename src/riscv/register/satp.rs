pub fn write(bits: usize) {
    unsafe {
        asm!("csrw satp, $0"
            :: "r"(bits)
            :: "volatile");
    }
}

pub fn set_root_table(mode: Mode, asid: usize, root_table_ppn: usize) {
    let bits: usize = ((mode as usize) << 31) | (asid << 22) | root_table_ppn;
    write(bits);
}

pub fn root_table_ppn() -> usize {
    let bits: usize = read();
    bits & (0xffc0_0000 - 1)
}

pub fn root_table_paddr() -> usize {
    return root_table_ppn() << 12
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

pub fn mode() -> Mode {
    let mode_bit: usize = read() >> 31;
    match mode_bit {
        0 => Mode::Bare,
        1 => Mode::Sv32,
        _ => panic!(),
    }
}

pub enum Mode {
    Bare = 0,
    Sv32 = 1,
}


