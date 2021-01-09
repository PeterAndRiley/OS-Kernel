OS-Kernel
===============================================================================

a system kernel.

What the project need?
-------------------------------------------------------------------------------

The project is builded by Rust Nightly.

> Why Rust Nightly instead of Rust Stable?
>
> This project need to use `#![feature(lang_items)]` to build a no-std OS (std
> need to build on another OS, which is impossible). But `#![feature(...)]`
> attributes are only allowed on the nightly release channel.
>
> Run `rustc --explain E0554` for more infomation.

This [document](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
is about Rust Nightly. Click it to know more infomation.

How to run it?
-------------------------------------------------------------------------------

Use command following to build it:
``` shell
cargo build
```

the target file is localed at `./target/debug/OS-Kernel`.

