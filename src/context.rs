use crate::riscv::register::scause::Scause;
use core::mem::zeroed;
use crate::riscv::register::sstatus;

#[repr(C)] // 表示对这个结构体按 C 语言标准 进行内存布局
#[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32], // General registers
    pub sstatus: usize, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: Scause, // Scause register: record the cause of exception/interrupt/trap
}

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}

#[repr(C)]
pub struct Context {
    content_addr: usize // 上下文内容存储的位置
}

/// 在发生函数调用时， riscv32 约定了 调用者保存寄存器（caller saved） 和
/// 被调用者保存寄存器（callee saved） ，保存前者的代码由编译器悄悄的帮我们生成，
/// 保存后者的代码则需要我们自己编写，所以结构体中只包含部分寄存器。
#[repr(C)]
struct ContextContent {
    ra: usize, // 返回地址
    satp: usize, //　二级页表所在位置
    s: [usize; 12], // 被调用者保存的寄存器
    tf: TrapFrame, // 中断帧
}


extern "C" {
    fn __trapret();
}

// 内核线程的 kstack 除了存放线程运行需要使用的内容，还需要存放 ContextContent 。
// 因此在创建 Thread 的时候，需要为其分配 kstack ，将 ContextContext 内容复制到 kstack 的 top 。
// 而 Context 只保存 ContextContent 首地址 content_addr
impl Context {
    pub unsafe fn null() -> Context {
        Context { content_addr: 0 }
    }

    pub unsafe fn new_kernel_thread(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        kstack_top: usize,
        satp: usize ) -> Context {
        ContextContent::new_kernel_thread(entry, arg, kstack_top, satp).push_at(kstack_top)
    }

    // 由于我们要完全手写汇编实现 switch 函数，因此需要给编译器一些特殊标记
    #[naked] // 表示不希望编译器产生多余的汇编代码。这里最重要的是 extern "C" 修饰，
    // 这表示该函数使用 C 语言的 ABI ，所以规范中所有调用者保存的寄存器（caller-saved）都会保存在栈上。
    #[inline(never)] // 禁止函数内联。这是由于我们需要 ret 和 ra 寄存器控制切换线程。如果内联了就没有 ret ，也就无法实现线程切换了。
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {//注意如何传参
        asm!(include_str!("process/switch.asm") :::: "volatile");
    }
}

impl ContextContent {
    fn new_kernel_thread(entry: extern "C" fn(usize) -> !, arg: usize , kstack_top: usize, satp: usize) -> ContextContent {
        let mut content: ContextContent = unsafe { zeroed() };
        content.ra = entry as usize; // sret 之后的权限模式和中断状态由 sstatus 控制，这一步指定跳转地址。
        content.satp = satp;
        content.s[0] = arg;
        let mut sstatus_ = sstatus::read();
        println!("sstatus_before: {}", sstatus_.bits());
        sstatus_.set_spp(sstatus::SPP::Supervisor); // 代表 sret 之后的特权级仍为 Ｓ
        println!("sstatus after: {}", sstatus::read().bits());
        println!("sstatus after: {}", sstatus_.bits());
        content.s[1] = sstatus_.bits();
        println!("sstatus saved: {}", content.s[1]);
        println!("sstatus after: {}", sstatus::read().bits());
        content
    }

    // TODO
    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self; // 拷贝 ContextContent
        Context { content_addr: ptr as usize }
    }
}