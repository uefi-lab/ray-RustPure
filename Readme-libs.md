
## 一键三连

r-efi
   包：   https://crates.io/crates/r-efi
   代码： https://github.com/r-efi/r-efi
   文档：
  
  底层数据映射, 仅仅就是 C -to- Rust 映射
  Rust 语言的 uefi_std 就是用的 r-efi
  不是给人日常用的, 类似基础设置, 给工具库用的, 交换数据结构.

mu_pi
  包：   https://crates.io/crates/mu_pi
  代码:  https://github.com/microsoft/mu_rust_pi
  文档:

  类似 r-efi 目的，只是把基础数据, 拓展到了 PI spec

uefi_std
  包:  语言特性. 打开  #![feature(uefi_std)]
  代码:   
  文档: https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html#rust-std-for-uefi

   Rust 语言的 std 库, 在 UEFI 环境下的实现.
   使用 r-efi 做 基础数据映射.

uefi-rs
  包：   https://crates.io/crates/uefi
  代码： https://github.com/rust-osdev/uefi-rs
  文档：

  自由行, 没有用 r-efi 自己手搓的 基础数据映射.
  目标是, 服务写 app 与 os loader 吧? 

patina
  包： https://crates.io/crates/patina

  目标是, 用 Rust 重写 DxeCore 模块.

  使用 r-efi 做 基础数据映射.

## 其他 no_std

