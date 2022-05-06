# Project rCore

The project rCore implementation by justxuewei.

## Contents

- [os](./src/os): rCore operatering system running on riscv64.
- [user](./src/user): user applications.
- [ch1-exercise1-ls](./src/ls): ls program, [reference](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/7exercise.html).
- [ch1-exercise2-backtrace](./src/backtrace): (WIP: encountered segmentation fault) backtrace using base pointer and frame pointer running on Linux x64, [reference](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/7exercise.html).
- ch2-exercise1-backtrace: implemented at [stack_trace.rs](./src/os/src/stack_trace.rs).
- ch2-exercise2-get-task-info: implemented at [os: syscall/task_info.rs](./src/os/src/syscall/task_info.rs) and [user: 05get_task_info.rs](./src/user/src/bin/05get_task_info.rs).

## Tips

VSCode configs for riscv64 target to avoid `#![no_std]` warning.

```
// ref: https://github.com/rust-lang/vscode-rust/issues/729
// for rust extenstion:
{
    "rust.target": "riscv64gc-unknown-none-elf",
    "rust.all_targets": false
}

// for rust-analyzer extenstion:
{
    "rust-analyzer.cargo.target": "riscv64gc-unknown-none-elf",
    "rust-analyzer.checkOnSave.allTargets": false
}
```

Recommended extensions for VSCode

- rust-analyzer
- GitLens
- RISC-V Support
- Markdown All in One
