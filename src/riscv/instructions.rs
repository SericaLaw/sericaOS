pub fn flush_tlb() {
    unsafe {
        asm!("sfence.vma"::::"volatile");
    }
}