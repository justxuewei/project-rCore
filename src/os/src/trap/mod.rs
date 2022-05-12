mod context;

use crate::syscall::syscall;

use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        // stvec 保存 trap 处理代码的入口地址
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // trap 原因
    let stval = stval::read(); // trap 附加信息
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // sepc 目前指向的是 ecall 指令的地址，但是它应该指向的是下一条指令，
            // 已知 ecall 指令的长度为 4，所以这里需要加 4。
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            // run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap: {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
