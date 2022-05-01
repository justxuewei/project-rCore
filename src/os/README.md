# rCore: An OS Running on RISC-V

这篇文章补充 [rCore-Tutorial-Book-v3](https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html) 中没有介绍到的知识，这些知识大部分来自于汇编、编译器等内容。

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
