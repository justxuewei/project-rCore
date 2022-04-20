#![no_std]
#![no_main]
#![feature(panic_info_message)]

// #[macro_use] 的作用是在 mod 作用域结束时依然可以使用 macro，
// 或者引入其他 crate 的 marcos。
// Ref: https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute
#[macro_use]

mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
// TODO(justxuewei): 在 rust 里加载 asm 的意义是什么？
// load entry.asm
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    println!("Hello, world!");
    panic!("Shutdown machine!")
}

fn clear_bss() {
    // TODO(justxuewei): sbss 和 ebss 这两个函数是在哪定义的？
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
