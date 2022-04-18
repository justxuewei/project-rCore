# .globl

.globl 是创建一个全局变量，比如下面代码表示创建一个 `_start` 变量。

```asm
    // -- snip --
    .globl _start
_start:
    // -- snip --
```

# .section

.section 语法表示将下面的代码放到二进制程序的某个段中。

- .text.entry: 表示代码段的最开头（整个程序的最开始）
- .bss.stack: 表示全局变量的 stack 段

# .space

.space 表示内存布局预留空间，`.space 4096 * 16` 表示预留 64KB 空间。
