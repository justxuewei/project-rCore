# User Applications

## linker.ld

我们的程序使用 linker.ld 脚本自定义了程序的内存布局，比如设置 BASE_ADDRESS 为 0x80400000，自定义程序的 section 布局等等。这里我比较好奇的是 BASE_ADDRESS 到底表示的是哪种地址，逻辑地址还是实际的物理地址？

当我们通过如下命令运行的时候，实际上是跑在 Linux on riscv64 中，BASE_ADDRESS 在我们非定制的 OS 上也能正常运行。

```bash
make build
# running with entire OS
qemu-riscv64 target/riscv64gc-unknown-none-elf/release/00hello_world
# running without OS: FAILED TO RUN
qemu-system-riscv64 target/riscv64gc-unknown-none-elf/release/00hello_world
```

这个问题的解答在 [内核第一条指令（实践篇）](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/4first-instruction-in-kernel2.html) 的“思考：0x80200000 可否改为其他地址？”中有提到过。

总结一下，这是要分情况的。我们的 rCore 目前处于相对早期的阶段（批处理阶段），不需要程序加载到内存的**任意地方**，所以 BASE_ADDRESS 表示的是内存的绝对地址。但是我们将程序运行在 Linux on riscv64 上时，是允许程序被加载到内存任意位置的，这种程序是位置无关文件（PIE, Position-independent Executable），那么 BASE_ADDRESS 的值就可以被随便设置，比如将 BASE_ADDRESS 的值设置为 `0x0`，在按照上面的方案命令，整个程序依然是可以正常运行的。
