#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U mode.");
    println!("Kernel should kill this application!");
    unsafe {
        // 从 S 模式返回 U 模式
        // Ref: https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/1rv-privilege.html#term-csr-instr
        asm!("sret")
    }
    0
}
