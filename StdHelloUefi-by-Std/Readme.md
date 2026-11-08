
使用 Rust 语言提供的 #![feature(uefi_std)]

1、简化 入口形式，变成 Rust 语言常见入口

2、不用自己写 #[panic_handler]

3、可以使用 语言的 标准形式

uefi_std 是, std 库 在 uefi 环境下的实现.


参考例子

  https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html#example-hello-world-with-std

  https://github.com/rust-osdev/uefi-rs/tree/main/uefi-std-example 


源代码位置
  
  library/std/src/os/uefi/env.rs   |  保存 Dxe 入口参数  // 就是讲把 DXE 环境, 认为是个 OS 

  library/std/src/sys/pal/uefi/mod.rs  | 貌似真正入口, 把 Dxe 入口参数, 存到 env 里面.
                                       | 牛掰啊, 最终用的还是 r-efi 最类型映射


  library/std/src/sys/stdio/uefi.rs  | 用 r-efi 的 simple_text_output 实现
  