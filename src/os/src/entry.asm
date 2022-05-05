    .section .text.entry
    
    .globl _start
_start:
    la sp, boot_stack_top # 在 OS 启动时候 sp 指向 boot_stack 的高地址，也就是 boot_stack_top
    call rust_main

    .section .bss.stack

    .globl boot_stack
boot_stack:
    .space 4096 * 16

    .globl boot_stack_top
boot_stack_top:
