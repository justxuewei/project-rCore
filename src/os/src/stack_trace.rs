use core::{arch::asm, ptr};

// riscv 函数内存模型结构详见
// https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/5support-func-call.html

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("fp = {:016x}", fp as usize);

    println!("===== Begin Stack Trace =====");
    while fp != ptr::null() {
        // QUESTION: 栈是从高向低增长的，fp 指针指向的是
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("ra = 0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }

    println!("===== End Stack Trace =====");
}
