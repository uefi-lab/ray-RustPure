
在当前目录，
  用 Rust 直接编译出 efi 文件  

     cargo build --target aarch64-unknown-uefi

  生成的文件在 
  
     ../target/aarch64-unknown-uefi/debug/hello-uefi.efi

备注：
  1. Rust 有个令人炸毛的，配置文件(们) 自动找爸爸 行为
     业界目前唯一可行的妥协是, 找一个 干净的爸爸 目录

  2. 爸爸目录里，折磨的死去活来的 workspace 可以靠，在当前 Cargo.toml
     里单独声明一次 [workspace] 摆脱关系

  3. 每个东西都有怪癖, 都有自己的 "哲学", 这种 "哲学" 
     权当笑话看看, 凑合起来 能工作就行.

-- Raymond, 2026-10-31


历史档案
    用 Rust 直接编译出 efi 文件

     cargo build --target aarch64-unknown-uefi --release

    生成的文件在

     HelloRustApp\target\aarch64-unknown-uefi\release\hello_uefi.efi"

    -- Raymond, 2026-01-15



参考例子, 代码

  r-efi 示例
  https://docs.rs/r-efi/latest/r_efi/

  Rust 语言 示例
  https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html

  Rust 语言 uefi_std 实现示例
  ……
  