# rCore

## linker.ld

已有的内容请参阅 [调整内核的内存布局](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/4first-instruction-in-kernel2.html#id4)，这里主要补充下我没有看懂的地方。

此外还有一些个人理解：

- [BASE_ADDRESS 解释](../user)

## sxx, exx 表示的含义

```
// -- snip --
skernel = .;
// -- snip --
ekernel = .;
// -- snip --
```

我理解这里的 `skernel` 和 `ekernel` 分别表示 kernel 开始的地方和结束的地方，因为 `.` 表示当前地址，所以这个变量表示的就是他们的起始地址，这里其他的 `s<x>` 和 `e<x>` 都是同理。

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

### ALIGN 关键词

Ref: https://zhuanlan.zhihu.com/p/383729996

执行的是对齐指令，这里不再详述。

## entry.asm

### .globl

.globl 是创建一个全局变量，比如下面代码表示创建一个 `_start` 变量。

```asm
    // -- snip --
    .globl _start
_start:
    // -- snip --
```

### .section

.section 语法表示将下面的代码放到二进制程序的某个段中。

- .text.entry: 表示代码段的最开头（整个程序的最开始）
- .bss.stack: 表示全局变量的 stack 段

### .space

.space 表示内存布局预留空间，`.space 4096 * 16` 表示预留 64KB 空间。##
