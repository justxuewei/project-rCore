#![no_std]
#![no_main]

mod lang_items;

use core::arch::global_asm;
// TODO(justxuewei): 在 rust 里加载 asm 的意义是什么？
// load entry.asm
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    loop {}
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
