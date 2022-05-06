#![no_std]
#![no_main]
#![feature(panic_info_message)]

// #[macro_use] 的作用是在 mod 作用域结束时依然可以使用 macro，
// 或者引入其他 crate 的 marcos。
// Ref: https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute
#[macro_use]
mod console;
pub mod batch;
mod lang_items;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;
mod stack_trace;
mod clock;

use core::arch::global_asm;

use embedded_time::{
    Clock,
    duration::Seconds,
};

// load entry.asm：让 RustSBI 知道 rCore 的入口函数是 rust_main
global_asm!(include_str!("entry.asm"));
// 将用户程序链接到操作系统中
global_asm!(include_str!("link_app.S"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    let instant1 = clock::SysClock.try_now().unwrap();
    let mut count: usize = 0;
    (0..1000000000).for_each(|i| {
        (0..1000000000).for_each(|j| {
            (0..1000000000).for_each(|k| {
                count += 1;
            });
        });
    });
    let instant2 = clock::SysClock.try_now().unwrap();
    let diff = instant2.checked_duration_since(&instant1).unwrap();
    let secs: Result<Seconds<u32>, _> = diff.try_into();
    println!("secs: {:?}", secs.unwrap());

    println!("[kernel] Welcome to rCore!");
    trap::init();
    batch::init();
    batch::run_next_app();
}

// clear_bss 初始化除了 kernel stack 以外的 .bss 区域
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        // Performs a volatile write of a memory location with 
        // the given value without reading or dropping the old 
        // value.
        // Ref: https://doc.rust-lang.org/std/ptr/fn.write_volatile.html
        unsafe { (a as *mut u8).write_volatile(0) }
    })
}
