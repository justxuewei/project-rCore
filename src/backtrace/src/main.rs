#![feature(type_ascription)]

use core::arch::asm;

// Refs: 
// 1. [Backtrace] https://techno-coder.github.io/example_os/2018/06/04/A-stack-trace-for-your-OS.html 
// 2. [Names of registers] http://tianyu-code.top/%E6%B1%87%E7%BC%96/%E5%AF%84%E5%AD%98%E5%99%A8%E4%BB%8B%E7%BB%8D/
// 3. [asm marco] https://doc.rust-lang.org/nightly/reference/inline-assembly.html
fn main() {
    function_a();
}

fn function_a() {
    println!("=== function a ===");
    function_b();
}

fn function_b() {
    println!("=== function b ===");
    function_c();
}

fn function_c() {
    println!("=== function c ===");
    backtrack();
}

fn backtrack() {
    let mut base_pointer: *const usize;

    unsafe {
        // Refs: #2, #3
        // mov dst, src: 把 src 的数据移动到 dst 中
        // rax: 累加器
        // rbp: base pointer
        // out("reg") expr: 把 reg 数据移到 expr 中
        asm!(
            "mov rax, rbp", 
            out("rax") base_pointer,
        )
    }

    // println!("base_pointer = {}", unsafe { *base_pointer } as usize);
    // println!("base_pointer.is_null() = {}", base_pointer.is_null());

    while !base_pointer.is_null() {
        let return_address = unsafe { *(base_pointer.offset(1)) } as usize;
        println!("call site = {}", return_address);
        // FIXME(justxuewei): the program will exit unexpectedly after
        // dereferenced the base_pointer.
        base_pointer = unsafe { *base_pointer } as *const usize;
        // println!("base_pointer = {}", unsafe { *base_pointer } as usize);
        // println!("base_pointer.is_null() = {}", base_pointer.is_null());
    }
}
