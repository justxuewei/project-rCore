# rCore: An OS Running on RISC-V

这篇文章补充 [rCore-Tutorial-Book-v3](https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html) 中没有介绍到的知识，这些知识大部分来自于汇编、编译器等内容。

## Basic Concepts

### Stack

栈是一块连续的内存区域，risc-v 中是由高地址向低地址增长的，栈顶是可以执行出栈、入栈操作的，栈底不能执行任何操作，sp 寄存器指向的是栈顶，fp 寄存器指向的是栈底。

### Registers on RISC-V

Ref: https://zhuanlan.zhihu.com/p/295439950

| Register | ABI Name | Desc | Saver |
| - | - | - | - |
| x0 | zero | Hard-wired zero | - |
| x1 | ra | Return address | Caller |
| x2 | sp | Stack Pointer | Callee |
| x5-x7 | t0-t2 | Temporaries | Caller |

### Debug with GDB

[rCore GDB 基础教程](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/4first-instruction-in-kernel2.html)

- x/10i 0x80000000 : 显示 0x80000000 处的10条汇编指令。
- x/10i $pc : 显示即将执行的10条汇编指令。
- x/10xw 0x80000000 : 显示 0x80000000 处的10条数据，格式为16进制32bit。
- info register: 显示当前所有寄存器信息。
- info r t0: 显示 t0 寄存器的值。
- break funcname: 在目标函数第一条指令处设置断点。
- break *0x80200000: 在 0x80200000 处设置断点。
- continue: 执行直到碰到断点。
- si: 单步执行一条汇编指令。

## Linker Script

链接器的作用是将不同的目标文件链接起来变成一个真正可执行的文件，一般称为 ELF (Executable and Linkable File)。已有的内容请参阅 [调整内核的内存布局](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/4first-instruction-in-kernel2.html#id4)，这里主要补充下原文中没有提到的部分。

此外还有一些个人理解：

- [BASE_ADDRESS 解释](../user)

## sxx, exx 表示的含义

这里只是原作者的一种命名方式，参见 Assembly > .globl。`skernel` 和 `ekernel` 分别表示 kernel 开始的地方和结束的地方，因为 `.` 表示当前地址，所以这个变量表示的就是他们的起始地址，这里其他的 `s<x>` 和 `e<x>` 都是同理。

```
/* -- snip -- */
skernel = .;
/* -- snip -- */
ekernel = .;
/* -- snip -- */
```

### 如何生成一个段

```
.text : {
    *(.text.entry)
    *(.text .text.*)
}
```

以 `*(.text.entry)` 为例，这里的通配符表示的是不同链接目标的 .text.entry 段，含义就是所有链接目标的 .text.entry 段都要放在 .text 和 .text.* 段的前面，那么最后生成的 .text 段可能的结构如下所示。

```
.text.entry         // o1
.text.entry         // o2
.text               // o1
.text.*             // o1
.text               // o2
.text.*             // o2
```

## Assembly

### .globl

.globl (.global) 是创建一个全局变量，全局变量是指对 linker 可见，这个全部变量只标志为一个名字，所以它可以是一个变量，也可以是一个数组，甚至可以是一个函数，总之这个变量与一个地址一一对应，具体内容是什么就要根据情况而定，它的地址是在链接后确定的，以 C 调用为例。

```
/* file: a.s */
    .section .data
    .globl phone_num
phone_num:
    .word 0x1234
    .word 0x5678
```

上面表示在代码段 (.data) 中创建一个全局变量 phone_num，它的值是 0x56781234，它的地址未知，需要等到全部链接后才能确定。

```c
// file: b.c
#include <stdio.h>

// 声明 a.s 中的全局变量 phone_num
extern unsigned int phone_num;

int main() {
    // 打印值
    printf("phone = 0x%x\n", phone_num);
    // 打印指针
    printf("&phone_num = %p\n", &phone_num);
}
```

最终的结果是

```
phone = 0x56781234
&phone_num = 0x601038
```

Refs:
- https://zhuanlan.zhihu.com/p/109474273
- https://blog.csdn.net/longintchar/article/details/80038843

### .section

.section 语法表示将下面的代码放到二进制程序的某个段中。

- .text.entry: 表示代码段的最开头（整个程序的最开始）
- .bss.stack: 表示全局变量的 stack 段

### .space

.space 表示内存布局预留空间，`.space 4096 * 16` 表示预留 64KB 空间。

### .align

对齐指令，在 riscv 中 .align == .p2align，比如 .align 3 可以理解为与 8 对齐，也就是填充一堆 0 直到当前地址可以整除 8，可以参考 [.p2align](https://sourceware.org/binutils/docs/as/P2align.html#P2align)。

### .altmacro & .rept

.altmacro 的作用是启动 alternate macro 模式，参考 [How to get the value of a variable in a macro argument](https://stackoverflow.com/questions/35214474/how-to-get-the-value-of-a-variable-in-a-macro-argument)，可以使用 `%expr` 获取表达式的值。

.rept 的作用是重复语句，可以参考 [.rept count](http://web.mit.edu/rhel-doc/3/rhel-as-en-3/rept.html)。

```
.set n, 5
.rept 27
    SAVE_GP %n
    .set n, n+1
.endr
```

就可以等效为如下汇编代码：

```
SAVE_GP 5
SAVE_GP 6
// ...
SAVE_GP 32
```

### .add & .addi & .sd

在 ch2 的 trap.S 中 addi 和 sd 指令是一起使用的，因此将这两个指令一起介绍。

add 和 addi 顾名思义都是做加法运算的，add 的两个加数是存放在寄存器中，addi 的一个加数存放在寄存器中，另一个加数是立即数。对于指令 add rd, rs, rt 可以表示为 rs + rt -> rd，对于指令 addi rd, rt, immediate 可以表示为 rt + immediate -> rd。

sd 是 store doubleword 的含义，与之相似的还有 sb = store byte、sh = store halfword 以及 sw = store word，其中 byte = 1 byte、halfword = 2 bytes、word = 4 bytes 以及 doubleword = 8 bytes。

addi 和 sd 之间的组合可以用于操作栈，如下面的汇编代码，具体含义如右边注释所示。

```
addi sp, sp, -34*8 # 开辟 8 * 34 bytes 空间，栈的增长是自高地址向低地址的
sd x1, 1*8(sp) # 将 x1 寄存器数据存到栈的第一个位置中
```

## System Clock

在 ch2-exercise4 中需要确定一个函数的执行时间，本来在有 OS 的基础上可以很容易借助内置的 std 包获取当前时间，在函数结束的时候与开始时间相减就可以了。但是在 no_std 环境下没有任何可用的系统时间，所以就导致大部分 crate 无法使用，经过调研选用 `embedding_time`，[API reference](https://docs.rs/embedded-time/latest/embedded_time/)。


