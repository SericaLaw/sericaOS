pub unsafe fn write(bits: usize) {
    asm!("csrwi sscratch, 0"::::"volatile");
}