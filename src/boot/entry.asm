    .section .text.entry
    .globl _start # 全局符号
_start:
    # load upper immediate
    lui sp, %hi(bootstacktop)   # 将栈指针 sp 置为栈顶地址

    call os_start

    .section .bss.stack
    # .align伪操作用于将当前PC地址推进到“2的integer次方个字节”对齐的位置。譬如“.align 3”即表示将当前PC地址推进到8个字节对齐的位置处。
    .align 12  # PGSHIFT
    .global bootstack
bootstack:
    .space 4096 * 4		        # 开辟一块栈空间（4个页）
    .global bootstacktop
bootstacktop:
