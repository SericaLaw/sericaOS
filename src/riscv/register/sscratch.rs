pub unsafe fn write(bits: usize) {
    asm!("csrw sscratch, x10"
            ::"{x10}"(bits)
            ::"volatile");
}