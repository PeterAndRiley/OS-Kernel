    .section .text.entry
    .globl _start
_start:
    # the begin of OS, which will be called by OpenSBI.

    # set t0 = boot_page_table's address(0xffff_ffff_8???_????, because of
    # `src/boot/linker64`) - 0xffff_ffff_0000_0000 to get its physical address,
    # and then shift 12 to get its page namber.
    lui t0,%hi(boot_page_table)
    li t1, 0x00000000ffffffff
    and t0, t0, t1
    srli t0, t0, 12

    # make `satp` register to use `Sv39` by set (8 << 60), and load the page
    # table's page number(because of `.align 12`, the frist item in that page
    # is the frist page table item), and refresh TLB
    li t1, (8 << 60)
    or t0, t0, t1
    csrw satp, t0
    sfence.vma

    # move the stack's pointer to sign `bootstack`
    la sp, boot_stack_top

    # call rust_main in `src/init.rs` to init OS
    call rust_main


    .section .bss.stack
    .align 12
    .global boot_stack
boot_stack:
    # 16KB memory space for OS's stack
    .space 4096 * 16


    .global boot_stack_top
boot_stack_top:
    # the end for OS's stack

    
    .section .data
    .align 12
boot_page_table:
    # the page table for virtual address -> physical address

    # 0th page table item
    # 0x0~0x4000_0000 -> invaild
    .quad 0

    # 1st page table item
    # 0x4000_0000~0x8000_0000 -> invaild
    .quad 0

    # 2nd page table item
    # 0x8000_0000~0xc000_000 -> 0x8000_0000~0xc000_000
    #
    # 1. [ref:0xcf] 0xcf(1100_1111): V(valid), R(readable), W(writeable),
    #               X(executable), A(accessed) and D(dirty) is all true, and
    #               G(global) and U(user page) is false
    # 2. [ref:<<10] Sv39 page table item: || Reserved(10) | PPN(44) |
    #               RSW(2) | status(8) ||, shift 10 bits to aviod RSW+status
    # 3. [ref:4KiB] The page's size is 4KiB = 4 * 1024 = 0x1000, so the page's
    #               index with 3 more 0s is the address
    # 4. [ref:giga page] if the R, W, X flag is not all 0, then it is the leaf
    #                    PTE, and when it is at the 0th page table, then it
    #                    point to the giga page, which has 4000_0000 bytes.
    .quad (0x80000 << 10) | 0xcf

    # 3rd page table item ~ 509th page table item
    # 0xc000_000~0x40_0000_0000 -> invaild
    # 0xffff_ffc0_0000_0000~0xffff_ffff_8000_0000 -> invaild
    #
    # 1. [ref:*8] Sv39's page table item have 8 bytes.
    # 2. [ref:Only39] Sv39 only have 39 vaild bits. [63:39] bits are same with
    #                 the 38th bit.
    .zero 507 * 8

    # 510th page table item
    # 0xffff_ffff_8000_0000~0xffff_ffff_c000_0000 -> 0x8000_0000~0xc000_000
    #
    # more infomations: [ref:0xcf], [ref:<<10], [ref:4KiB]
    .quad (0x80000 << 10) | 0xcf

    # 511th page
    # 0xffff_ffff_c000_0000~0x1_0000_0000_0000_0000 -> invaild
    .quad 0

