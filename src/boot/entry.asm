    .section .text.entry
    .globl _start # 全局符号
_start:
    # 1 paging
    # satp = (1 << 31) | PPN(boot_page_table_sv32)
    lui     t0, %hi(boot_page_table_sv32) # 将 boot_page_table_sv32 的高 20 位复制到 t0 的高 20 位，t0 的低 12 位填 0 。

    # boot_page_table_sv32 是个符号，使用虚地址。
    # 而 satp 中需要填入物理地址。因此需要减去一个偏移。
    li      t1, 0xC0000000 - 0x80000000
    sub     t0, t0, t1 # t0是page table物理地址, 4K对齐

    #  1.1 linear mapping (0xc0000000 -> 0x80000000)
    li      t2, 768*4
    li      t4, 0x400 << 10
    li      t5, 4
    add     t1, t0, t2
    li      t6, 1024*4
    add     t6, t0, t6
    li      t3, (0x80000 << 10) | 0xcf # VRWXAD
loop:
    sw      t3, 0(t1)
    add     t3, t3, t4
    add     t1, t1, t5
    bne     t1, t6, loop

    # recursive
    li      t2, 1023*4
    add     t1, t0, t2

    # t3 = (t0 >> 2) | 0x01
    mv      t3, t0
    srli    t3, t3, 2
    ori     t3, t3, 0x01
    sw      t3, 0(t1)

    li      t2, 1022*4
    add     t1, t0, t2

    # t3 = (t0 >> 2) | 0x0cf
    mv      t3, t0
    srli    t3, t3, 2
    ori     t3, t3, 0x0cf
    sw      t3, 0(t1)

    #  1.2 enable paging
    srli    t0, t0, 12  # 右移 12 位后就正确的给 satp.PPN 部分赋值了。
    li      t1, 1 << 31 # satp MODE = 1
    or      t0, t0, t1
    csrw    satp, t0 # set satp
    sfence.vma

    # 使用绝对跳转指令来切换 PC
    # 首先定义一个符号 remove_identity_map
    # 然后将其虚地址加载到寄存器 t0
    # 最后使用 jr 指令跳转到 t0 指向的地址。
    # 1.2 update PC to 0xCxxxxxxx
    lui     t0, %hi(remove_identity_map)
    addi    t0, t0, %lo(remove_identity_map)
    jr      t0

    # 现在 PC 已经指向位于 0xC0000000 区域的虚拟地址空间，就可以撤销对等映射了
    # 我们向下标为 513 的页表项地址处写入 0, 刷新 TLB
    # 1.3 remove identity map
remove_identity_map:
    lui     t0, %hi(boot_page_table_sv32_top)
    sw      zero, (-4 * 511)(t0)


    sfence.vma # 刷新 TLB

    # 我们需要一段同时存在两种映射的过渡期来切换 PC ，之后就可以取消这个对等映射。
    # 2. setup stack pointer
    # load upper immediate
    lui sp, %hi(bootstacktop)   # 将栈指针 sp 置为栈顶地址

    # 3. call os_start
    call os_start

    .section .bss.stack
    # .align伪操作用于将当前PC地址推进到“2的integer次方个字节”对齐的位置。譬如“.align 3”即表示将当前PC地址推进到8个字节对齐的位置处。
    .align 12  # PGSHIFT
    .global bootstack
bootstack:
    .space 4096 * 4		        # 开辟一块栈空间（4个页 4KB * 4）
    .global bootstacktop
bootstacktop:

# OpenSBI 所在内存不再需要被映射，然后将 0xC0400000 内存映射到 0x80400000 。
# 注意，这里保留了 0x80400000 内存映射到 0x80400000 的对等映射
# 之所以还要保留 0x80400000 的对等映射，是因为在开启分页的一瞬间，PC 仍然指向物理地址。
# 如果撤掉了对等映射，那么在设置 satp 的下一条指令会立即触发缺页异常。
    .section .data
    .align 12   # 4K 页对齐
boot_page_table_sv32: # a 4KB page
    .zero 4 * 64
    .word (0x10000 << 10) | 0xcf
    .zero 4 * 448
    # .zero 4 * 513 # nothing
    # 0x80400000 -> 0x80400000 (4M)
    .word (0x80400 << 10) | 0xcf # VRWXAD
    .zero 4 * 255
    # 0xC0400000 -> 0x80400000 (4M)
    .word (0x80400 << 10) | 0xcf # VRWXAD
    # 0xC0800000 -> 0x80800000 (4M)
    .word (0x80800 << 10) | 0xcf # VRWXAD
    .zero 4 * 253
boot_page_table_sv32_top:
