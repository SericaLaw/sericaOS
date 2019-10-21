pub unsafe fn read() -> usize {
    let mut lo: usize;
    asm!("rdtime x10"
            :"={x10}"(lo)
            :::"volatile");
    lo
}