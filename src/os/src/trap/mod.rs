mod context;

use crate::{config, syscall::syscall, task, timer};
use core::arch::{global_asm, asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(config::TRAMPOLINE as usize, TrapMode::Direct);
    }
}

// 启用 timer 中断
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = task::current_trap_cx();
    let scause = scause::read(); // trap 原因
    let stval = stval::read(); // trap 附加信息
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // sepc 目前指向的是 ecall 指令的地址，但是它应该指向的是下一条指令，
            // 已知 ecall 指令的长度为 4，所以这里需要加 4。
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            task::exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            task::exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // println!("[kernel debug] time interrupt is fired");
            timer::set_next_trigger();
            task::suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap: {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    
    trap_return();
}

#[no_mangle]
// 用于从内核态切换为用户态，并在用户态调用 __restore 方法
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = config::TRAP_CONTEXT;
    let user_token = task::current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + config::TRAMPOLINE;

    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_token,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel")
}
