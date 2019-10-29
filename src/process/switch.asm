.equ XLENB, 4
.macro Load reg, mem
    lw \reg, \mem
.endm
.macro Store reg, mem
    sw \reg, \mem
.endm

    addi  sp, sp, (-XLENB*14)

    # 将用于保存上下文地址的 content_addr 赋值给 sp
    # 然后在 sp 指向的位置保存 callee saved 寄存器。
    Store sp, 0(a0)
    Store ra, 0*XLENB(sp)
    Store s0, 2*XLENB(sp)
    Store s1, 3*XLENB(sp)
    Store s2, 4*XLENB(sp)
    Store s3, 5*XLENB(sp)
    Store s4, 6*XLENB(sp)
    Store s5, 7*XLENB(sp)
    Store s6, 8*XLENB(sp)
    Store s7, 9*XLENB(sp)
    Store s8, 10*XLENB(sp)
    Store s9, 11*XLENB(sp)
    Store s10, 12*XLENB(sp)
    Store s11, 13*XLENB(sp)
    csrr  s11, satp
    Store s11, 1*XLENB(sp)

    # 将目标线程的 content_addr 赋值给 sp ，然后恢复目标线程的寄存器。
    Load sp, 0(a1)
    Load s11, 1*XLENB(sp)
    csrw satp, s11
    Load ra, 0*XLENB(sp)
    Load s0, 2*XLENB(sp)
    Load s1, 3*XLENB(sp)
    Load s2, 4*XLENB(sp)
    Load s3, 5*XLENB(sp)
    Load s4, 6*XLENB(sp)
    Load s5, 7*XLENB(sp)
    Load s6, 8*XLENB(sp)
    Load s7, 9*XLENB(sp)
    Load s8, 10*XLENB(sp)
    Load s9, 11*XLENB(sp)
    Load s10, 12*XLENB(sp)
    Load s11, 13*XLENB(sp)
    mv a0, s0 # 传入新线程的参数
    addi sp, sp, (XLENB*14)

    Store zero, 0(a1)
    ret