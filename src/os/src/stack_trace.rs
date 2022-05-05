use core::{arch::asm};

// riscv 函数内存模型结构详见
// https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/5support-func-call.html

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);

    println!("===== Begin Stack Trace =====");

    // 原教程的参考答案中使用的是如下代码，但是我在验证过程中发现会出现 LoadFault 问题
    // 经过验证是在最后的时候 *fp 已经指向 0x0 位置，而继续执行 *fp.sub(count) 会造
    // 成非法访问的问题，所以我在这里修改了源代码如下所示。
    // Ref: https://github.com/rcore-os/rCore-Tutorial-Book-v3/issues/116#issuecomment-1118424671
    // while fp != ptr:: null() {}
    while !(*fp as *const usize).is_null() {
        // println!("fp = {:016x}, *fp = {:016x}, fp.sub(2) = {:016x}, *fp.sub(2) = {:016x}", fp as usize, *fp, fp.sub(2) as usize, *fp.sub(2));
        // QUESTION: 栈是从高向低增长的，fp 指针指向的是
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("ra = 0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }

    println!("===== End Stack Trace =====");
}
