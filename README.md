# Project rCore

The project rCore implementation by justxuewei.

## Contents

- [os](./src/os): rCore operatering system running on riscv64.
- [user](./src/user): user applications.
- [ch1-exercise1-ls](./src/ls): ls program, [reference](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/7exercise.html).
- [ch1-exercise2-backtrace](./src/backtrace): (WIP: encountered segmentation fault) backtrace using base pointer and frame pointer running on Linux x64, [reference](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/7exercise.html)

## Tips

VSCode configs for riscv64 target to avoid `#![no_std]` warning.

```
// ref: https://github.com/rust-lang/vscode-rust/issues/729
// for rust extenstion:
{
    "rust.target": "thumbv7em-none-eabihf",
    "rust.all_targets": false
}

// for rust-analyzer extenstion:
{
    "rust-analyzer.cargo.target": "thumbv7em-none-eabihf",
    "rust-analyzer.checkOnSave.allTargets": false
}
```
