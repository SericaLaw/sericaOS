/* 目标体系结构riscv，入口符号地址_start */
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0xC0400000;
/*  use the ‘SECTIONS’ command to describe the memory layout of the output file. */
SECTIONS
{
    /* 定位符设置为该基址 */
    . = BASE_ADDRESS;

/* text: Program code section */
    .text : {
        /* text 入口 */
        *(.text.entry)

        *(.text .text.*)
        . = ALIGN(4K);
    }
  /* rodata: Read-only data */
    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }
  /* End of code and read-only segment */
  /* data: Writable, initialized data */    
  .data : {
        *(.data .data.*)
    }

    .stack : {
        *(.bss.stack)
    }
  /* bss: Uninitialized writeable data section */
    .bss : {
        *(.bss .bss.*)
    }
  /* end od program */
    PROVIDE(end = .);
}
