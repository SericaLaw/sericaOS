#![allow(dead_code)]

// Extension IDs, see https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc#function-listing-1
const SBI_SET_TIMER: usize = 0x00;
const SBI_CONSOLE_PUTCHAR: usize = 0x01;
const SBI_CONSOLE_GETCHAR: usize = 0x02;
const SBI_CLEAR_IPI: usize = 0x03;
const SBI_SEND_IPI: usize = 0x04;
const SBI_REMOTE_FENCE_I: usize = 0x05;
const SBI_REMOTE_SFENCE_VMA: usize = 0x06;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 0x07;
const SBI_SHUTDOWN: usize = 0x08;

// Error Code
const SBI_SUCCESS: isize  = 0;
const SBI_ERR_FAILURE: isize = -1;
const SBI_ERR_NOT_SUPPORTED: isize = -2;
const SBI_ERR_INVALID_PARAM: isize = -3;
const SBI_ERR_DENIED: isize = -4;
const SBI_ERR_INVALID_ADDRESS: isize = -5;

// according to risc-v reader, 10 = a0, x11 = a1, x12 = a2, x17 = a7
// according to risc-v sbi doc, a7 encodes the SBI extension ID
#[inline(always)]
fn sbi_call(func: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!("ecall"
            : "={x10}" (ret) // output
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (func) // input
            : "memory"
            : "volatile");
    }
    ret
}

// Programs the clock for next event after stime_value time. This function also clears the pending timer interrupt bit.
pub fn set_timer(stime_value: u64) {
    #[cfg(target_pointer_width = "32")]
    sbi_call(
    SBI_SET_TIMER,
    stime_value as usize,
    (stime_value >> 32) as usize,
    0,
    );
    #[cfg(target_pointer_width = "64")]
    sbi_call(
    SBI_SET_TIMER,
     stime_value as usize,
     0,
     0
    );
}

// Write data present in ch to debug console.
//
// Unlike sbi_console_getchar, this SBI call will block if there remain any pending characters to be transmitted or if the receiving terminal is not yet ready to receive the byte. However, if the console doesn’t exist at all, then the character is thrown away.
pub fn console_putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

// Read a byte from debug console; returns the byte on success, or -1 for failure. Note. This is the only SBI call that has a non-void return type.
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

// Puts all the harts to shut down state from supervisor point of view. This SBI call doesn’t return.
pub fn shutdown() {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
}

pub fn clear_ipi() {
    sbi_call(SBI_CLEAR_IPI, 0, 0, 0);
}

pub fn send_ipi(hart_mask: usize) {
    sbi_call(SBI_SEND_IPI, &hart_mask as *const _ as usize, 0, 0);
}

pub fn remote_fence_i(hart_mask: usize) {
    sbi_call(SBI_REMOTE_FENCE_I, &hart_mask as *const _ as usize, 0, 0);
}

pub fn remote_sfence_vma(hart_mask: usize, _start: usize, _size: usize) {
    sbi_call(SBI_REMOTE_SFENCE_VMA, &hart_mask as *const _ as usize, 0, 0);
}

pub fn remote_sfence_vma_asid(hart_mask: usize, _start: usize, _size: usize, _asid: usize) {
    sbi_call(
        SBI_REMOTE_SFENCE_VMA_ASID,
        &hart_mask as *const _ as usize,
        0,
        0,
    );
}


