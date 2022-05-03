mod context;

use crate::sys

use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMod,
    scause::{self, Exception, Trap},
    stval, stvec
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        // stvec 保存 trap 处理代码的入口地址
        stvec::write(__alltraps as usize, TrapMod::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();    // trap 原因
    let stval = stval::read();      // trap 附加信息
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = 
        }
    }
}
