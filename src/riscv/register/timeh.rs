pub unsafe fn read() -> usize {
    let mut hi: usize;
    asm!("rdtimeh x10"
            :"={x10}"(hi)
            :::"volatile");
    hi
}