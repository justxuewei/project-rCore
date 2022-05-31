.altmacro

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    
    .align 2                # the address of __alltraps must be aligned in 2^2 bytes
# switch kerner stack from user stack and save all rigsters
__alltraps:
    # sp -> user stack & sscratch -> *TrapContext in user space
    csrrw sp, sscratch, sp  # 交换 sp 和 sscratch 的值
    # sp -> *TrapContext in user space & sscratch -> user stack
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
    sd t0, 32*8(sp) # 将 sstatus 保存到 TrapContext::sstatus
    sd t1, 33*8(sp) # 将 sepc 保存到 TrapContext::sepc
    csrr t2, sscratch
    sd t2, 2*8(sp)  # 将 user stack 保存到 TrapContext::x[2]
    ld t0, 34*8(sp) # 将 TrapContext::kernel_stap 保存到 t0 寄存器
    ld t1, 36*8(sp) # 将 TrapContext::trap_handler 保存到 t1 寄存器
    ld sp, 35*8(sp) # 将 TrapContext::kernel_stack 保存到 sp 寄存器，此时切换为 kernel stack
    csrw satp, t0   # 使用内核虚拟空间
    sfence.vma
    jr t1           # 跳转到 trap_handler

# 在执行完 trap_handler 之后会按照顺序继续执行 __restore

# case1: start running app by __restore
# case2: back to U Mode after handling trap
__restore:
    # a0 -> *TrapContext in user space; a1 -> user space token
    csrw satp, a1       # 使用用户内核空间
    sfence.vma
    csrw sscratch, a0   # sscratch -> *TrapContext in user space
    mv sp, a0           # sp -> *TrapContext in user space
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
    ld sp, 2*8(sp)      # sp -> TrapContext::x[2], aka user stack
    sret
