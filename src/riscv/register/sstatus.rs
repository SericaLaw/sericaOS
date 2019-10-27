pub unsafe fn set_sie() {
    // TODO: 理论上可以换成另一条指令一步到位，但好像不work
    let mut sstatus: usize;
    asm!("csrr x10, sstatus"
            :"={x10}"(sstatus)
            :::"volatile");
    sstatus |= 0b10;
    asm!("csrw sstatus, x10"
                    ::"{x10}"(sstatus)
                    ::"volatile");
}

pub unsafe fn set_sum() {
    let mut sstatus: usize;
    asm!("csrr x10, sstatus"
            :"={x10}"(sstatus)
            :::"volatile");
    sstatus |= 1 << 18;
    asm!("csrw sstatus, x10"
                    ::"{x10}"(sstatus)
                    ::"volatile");
}