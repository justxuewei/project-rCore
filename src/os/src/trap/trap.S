.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
