#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[no_mangle]
#[linkage = "weak"]
// 这里 weak 是告诉 linker 这个 main 是可以被
// 其他 main 函数覆盖的，这里加一个 main 主要是
// 为了防止 bin 中缺失 main 函数导致编译失败。
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
