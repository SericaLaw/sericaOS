/// x17 寄存器保存了用户产生的系统调用的编号，x10~x13 寄存器保存了系统调用的参数。
/// 用户态产生系统调用后会进入内核态，在内核态中对产生的 syscall 进行处理
#[inline(always)]
fn sys_call(
    syscall_id: SyscallId,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> i32 {
    let id = syscall_id as usize;
    let mut ret: i32;
    unsafe {
        asm!("ecall"
            : "={x10}" (ret)
            : "{x17}" (id), "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x13}" (arg3)
            : "memory"
            : "volatile");
    }
    ret
}

pub fn sys_write(ch : u8) -> i32 {
    sys_call(SyscallId::Write, ch as usize, 0, 0, 0)
}

pub fn sys_exit(code: usize) -> ! {
    sys_call(SyscallId::Exit, code, 0, 0, 0);
    loop{}
}

enum SyscallId {
    Write = 64,
    Exit = 93,
}