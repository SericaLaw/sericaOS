/* 目标体系结构riscv，入口符号地址_start */
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0xC0400000;
/*  use the ‘SECTIONS’ command to describe the memory layout of the output file. */
SECTIONS
{
    /* 定位符设置为该基址 */
    . = BASE_ADDRESS;
    start = .;

/* text: Program code section */
    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }
  /* rodata: Read-only data */
    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }
  /* End of code and read-only segment */
  /* data: Writable, initialized data */
    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }

    .stack : {
        *(.bss.stack)
    }

 /* bss: Uninitialized writeable data section */
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }

    PROVIDE(end = .);
}