.equ XLENB,     4
.macro LOAD a1, a2 # 读取内存地址 sp+a2*4 的值到寄存器 a1
    lw \a1, \a2*XLENB(sp)
.endm
.macro STORE a1, a2  # 将寄存器 a1 的值保存到内存地址 sp+a2*4
    sw \a1, \a2*XLENB(sp)
.endm

.macro SAVE_ALL
    # If coming from userspace, preserve the user stack pointer and load
    # the kernel stack pointer. If we came from the kernel, sscratch
    # will contain 0, and we should continue on the current stack.
    # 中断可能来自用户态（U-Mode），也可能来自内核态（S-Mode）。
    # 如果是用户态中断，那么此时的栈指针 sp 指向的是用户栈；
    # 如果是内核态中断，那么 sp 指向的是内核栈。
    # 我们希望把寄存器保存在内核栈上， 这就要求有一个通用寄存器指向内核栈。
    # 对于内核态中断来说，直接使用 sp 就可以了，
    # 但对于用户态中断，我们需要在不破坏 32 个通用寄存器的情况下，切换 sp 到内核栈。
    # 解决问题的关键是要有一个可做交换操作的临时寄存器，这里就是 sscratch 。
    # 我们规定：当 CPU 处于 U-Mode 时，sscratch 保存内核栈地址；
    # 处于 S-Mode 时，sscratch 为 0

    csrrw sp, sscratch, sp  #  交换 sp 和 sscratch 寄存器

    # 判断 sp（也就是交换前的 sscratch）是否为0
    # 如果非0，说明是用户态中断，由于 sscratch 保存的是内核栈地址
    # 此时 sp 已经指向内核栈，直接跳转到 trap_from_user 保存寄存器
    bnez sp, trap_from_user
    # 否则说明是内核态中断
trap_from_kernel:
    # 只需从 sscratch 中读出原来的 sp 即可
    csrr sp, sscratch
    # 此时 sscratch = 发生中断前的 sp sp = 内核栈
trap_from_user:
    # 此时 sp 指向的是内核栈的栈底，接下来我们要把 TrapFrame 保存在栈上，因此先将它下移相当于 TrapFrame 大小的距离（36 * 4 Byte），
    # 然后依次保存除 x0(zero) 和 x2(sp) 外的 30 个通用寄存器
    # provide room for trap frame
    addi sp, sp, -36*XLENB
    # save x registers except x2 (sp)
    # x0 永远是 0，不用保存；sp 本应保存的是发生中断前的值，这个值目前被交换到了 sscratch 中，因此留到后面处理。
    STORE x1, 1
    STORE x3, 3
    STORE x4, 4
    STORE x5, 5
    STORE x6, 6
    STORE x7, 7
    STORE x8, 8
    STORE x9, 9
    STORE x10, 10
    STORE x11, 11
    STORE x12, 12
    STORE x13, 13
    STORE x14, 14
    STORE x15, 15
    STORE x16, 16
    STORE x17, 17
    STORE x18, 18
    STORE x19, 19
    STORE x20, 20
    STORE x21, 21
    STORE x22, 22
    STORE x23, 23
    STORE x24, 24
    STORE x25, 25
    STORE x26, 26
    STORE x27, 27
    STORE x28, 28
    STORE x29, 29
    STORE x30, 30
    STORE x31, 31
    # 接下来保存 CSR 寄存器，此时通用寄存器已经可以随意使用了
    # 读取sp, sstatus, sepc, stval, scause
    # 按照规定，进入内核态后 sscratch 应为 0，set sscratch = 0
    csrrw s0, sscratch, x0
    csrr s1, sstatus
    csrr s2, sepc
    csrr s3, stval
    csrr s4, scause
    # 保存 sp, sstatus, sepc, sbadvaddr, scause
    STORE s0, 2
    STORE s1, 32
    STORE s2, 33
    STORE s3, 34
    STORE s4, 35
.endm

# 恢复寄存器 RESTORE_ALL 的过程正好相反。首先根据 sstatus 寄存器中的 SPP 位，判断是回到用户态还是内核态。
# 如果是回到用户态，根据规定需要设置 sscratch 为内核栈：
.macro RESTORE_ALL
    LOAD s1, 32             # s1 = sstatus
    LOAD s2, 33             # s2 = sepc
    andi s0, s1, 1 << 8     # sstatus.SPP = 1?
    bnez s0, _to_kernel     # s0 = back to kernel?
_to_user:
    addi s0, sp, 36*XLENB
    csrw sscratch, s0         # sscratch = kernel-sp
_to_kernel:
    # restore sstatus, sepc
    csrw sstatus, s1
    csrw sepc, s2

    # restore x registers except x2 (sp)
    LOAD x1, 1
    LOAD x3, 3
    LOAD x4, 4
    LOAD x5, 5
    LOAD x6, 6
    LOAD x7, 7
    LOAD x8, 8
    LOAD x9, 9
    LOAD x10, 10
    LOAD x11, 11
    LOAD x12, 12
    LOAD x13, 13
    LOAD x14, 14
    LOAD x15, 15
    LOAD x16, 16
    LOAD x17, 17
    LOAD x18, 18
    LOAD x19, 19
    LOAD x20, 20
    LOAD x21, 21
    LOAD x22, 22
    LOAD x23, 23
    LOAD x24, 24
    LOAD x25, 25
    LOAD x26, 26
    LOAD x27, 27
    LOAD x28, 28
    LOAD x29, 29
    LOAD x30, 30
    LOAD x31, 31
    # restore sp last
    LOAD x2, 2
.endm

.section .text
.globl __alltraps
__alltraps:
    # 保存所有的寄存器的状态
    SAVE_ALL
    # a0 是 riscv32 中的参数寄存器，用于存放下一个调用的函数的参数。
    # 将 a0 赋值为 sp ，也就是栈帧的地址。这样便成功的将栈帧作为参数传递给了 rust_trap 。
    mv a0, sp
    jal rust_trap

# 通过 sret 指令完成中断返回：CPU 根据 sstatus.SPP 确定特权级，将 sepc 恢复到 PC 。
.globl __trapret
__trapret:
    # 恢复所有的寄存器的状态
    RESTORE_ALL
    # return from supervisor call
    sret