/* Copy from bbl-ucore : https://ring00.github.io/bbl-ucore      */

/* Simple linker script for the ucore kernel.
   See the GNU ld 'info' manual ("info ld") to learn the syntax. */

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* 0xffffffff80000000 ~ 0xffffffff80200000: OpenSBI */
/* 0xffffffff80200000 ~ ...: OS-Kernel */
BASE_ADDRESS = 0xffffffff80200000;

SECTIONS
{
    /* Load the kernel at this address: "." means the current address */
    . = BASE_ADDRESS;
    start = .;

    . = ALIGN(4K);
    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    . = ALIGN(4K);
    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    . = ALIGN(4K);
    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }

    . = ALIGN(4K);
    .stack : {
        *(.bss.stack)
    }

    . = ALIGN(4K);
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }

    . = ALIGN(4K);
    PROVIDE(end = .);
}
