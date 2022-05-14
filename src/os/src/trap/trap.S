.altmacro

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    
    .align 2                # the address of __alltraps must be aligned in 2^2 bytes
# switch kerner stack from user stack and save all rigsters
__alltraps:
    # sp -> user stack & sscratch -> kernel stack
    csrrw sp, sscratch, sp  # 将 sscratch 读取到 sp，将 sp 写入 sscartch，也就是交换 sscratch 和 sp 的值
    # sp -> kernel stack & sscratch -> user stack
    addi sp, sp, -34*8      # sizeof TrapContext == 34 * 8
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # t0-t2 -> x5-x7
    # 所以 t0-t2 寄存器的内容已经被保存了，可以自由使用
    # ref: https://zhuanlan.zhihu.com/p/295439950
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv a0, sp           # 在执行该指令后 kernel stack sp 会作为 trap_handler 的入参
    call trap_handler

# 在执行完 trap_handler 之后会按照顺序继续执行 __restore

# case1: start running app by __restore
# case2: back to U after handling trap
__restore:
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)      # t2 保存的是 user stack
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2   # sscratch -> user stack
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp  # 交换 sp 和 sscratch 的值，现在 sp -> user stack & sscratch -> kernel stack
    sret
