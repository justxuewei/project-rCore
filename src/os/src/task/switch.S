.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a0)
.endm

    .section .text
    .globl __switch
# __switch(crt_tcx: *mut TaskContext, next_tcx: *const TaskContext)
# tcx 是 task_context 的缩写
# a0 表示 current task, a1 表示 next task
__switch:
    sd sp, 8(a0)
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    # =====
    # so far, all registers of current task were saved,
    # loading next task...
    # =====
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    ld sp, 8(a1)
    ret
