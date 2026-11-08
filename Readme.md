
## Pure Rust "Hello World" for UEFI


| Example | Folder| Comment |
|---------|-------|---------|
hello-uefi     | HelloUefiPureRust             | 裸奔 Rust lang + r-efi
hello-uefi-rs  | HelloUefi-by-UefiRs           | 利用 uefi-rs 
hello-uefi-std | StdHelloUefi-by-Std           | 利用  `#![feature(uefi_std)]` 语言内建
hello-uefi-rs-with-std | StdHelloUefi-by-UefiRsWithStd | uefi-rs + uefi_std
hello-uefi-patina  | HelloUefi-by-Patina     | (手搓出 Osmosis 之后) 硬凑出来的 案例
hello-uefi-mu (Not Ready) | HelloUefi-by-Mu  | (实在累了) 先不凑吧，MU 都转到 Patina 了

=== 


用 Rust 直接编译出 efi 文件

    cargo build --target aarch64-unknown-uefi -p hello-uefi
    cargo build --target aarch64-unknown-uefi -p hello-uefi-rs
    cargo build --target aarch64-unknown-uefi -p hello-uefi-std
    cargo build --target aarch64-unknown-uefi -p hello-uefi-rs-with-std
    cargo build --target aarch64-unknown-uefi -p hello-uefi-patina 

生成的文件在

    target\aarch64-unknown-uefi\debug\hello_uefi.efi

==

问题：我这些 package 的配置 feature 什么的都不同，输出到同一个 target 目录，会有风险嘛？  

  Copilot 的回答, 不知所云

==

不能执行 workspace 层次的 build

    cargo build --target aarch64-unknown-uefi --workspace

  因为 Rust 的 workspace 很麻烦, 有个（对我们）恶心的 Cargo feature unification 功能  
  当前（所谓的） workspace 下, 配置了不同的 language feature 参数, 目前 cargo 不允许  
  
>  当前弄出个 workspace 来，主要是为了 **打断** 父目录自动爬升 行为.  
>  
>  不是要弄这么复杂, 我作为 初学者, 根本不想这么弄.  
>  但是, 不这么弄, 代码行为完全不可 预期, 父目录可以轻而易举打断 初学者的 正常代码.  

  Cargo 的 workspace 更像一个超大的 package  


## 令人发指的, 父目录自动爬升找设置，一群文件都这样！

    Cargo.toml            # workspace
    .cargo/config.toml    # build config
    rust-toolchain.toml   # toolchain
    rustfmt.toml          # rust formatter
    clippy.toml           # toml formatter
    taplo.toml            # lint
    Makefile.toml         # task runner

  一个不小心, 就被坑了, 也不知道将来还会用什么 文件.

  目前只能，靠 大脑 + 手工 + 小心翼翼 维护，脆弱不堪.  
  一个 git submodule 就能 轻易的打碎, 目前没有 根治方法.

> 初学者，最好再 建立个顶级目录学习 Rust, 不要在现有工程里 建立子目录.

## 坑爹的案例（实际 | 自动找爸爸）

对于下面这种布局,   
（1）实际上 `HidIo` 只会用 MU workspace    
（2）实际上 `UefiHidDxeV2` **也只会用** MU workspace    

```txt
SomeProject/
├─ Cargo.toml         <-- 你的 workspace
└─ Common/
   └─ MU/
      ├─ Cargo.toml   <-- MU workspace
      └─ HidPkg/
          ├─ Crates/
          |  └─ HidIo/
          └─ UefiHidDxeV2/
```

```toml
# SomeProject/Cargo.toml

[workspace]
  resolver = "3"
  
  members = [
    "Common/MU/HidPkg/UefiHidDxeV2",
  ]

[workspace.dependencies]
  #
  HidIo = { path = "Common/MU/HidPkg/Crates/HidIo" }

```

    cargo metadata --manifest-path Common/MU/HidPkg/UefiHidDxeV2/Cargo.toml --no-deps --format-version 1 | jq -r '.workspace_root'

    cargo metadata --manifest-path Common/MU/HidPkg/Crates/HidIo/Cargo.toml --no-deps --format-version 1 | jq -r '.workspace_root'

结论：自动找爸爸，目录结构 的爸爸最大。 2026-11-08

<del>
也就是讲, Cargo 是有机制抢夺 workspace 控制权的?

貌似，这里 HidIo 的设置，是错的，大错特错的啊?  
按照意图来看，是错的！ 因为都是从源代码构建的啊？  

难道, 加入 members 能影响 EDK2 的行为?  
因为 UefiHidDxeV2 是混血的, 里面有个 INF 文件?  
没道理啊? INF 走的另一套 DSC 机制啊? 只是 pack-time 有效  
[compile|link]-time 走的都是 rust 机制啊?  

</del>


## 一键三连






find . -name Cargo.lock -delete

Get-ChildItem -Recurse -Filter Cargo.lock | Remove-Item -Force

Get-ChildItem -Recurse -Directory -Filter target | Remove-Item -Recurse -Force


## HelloUefi-by-UefiRs

https://github.com/rust-osdev/uefi-rs/tree/main/uefi-std-example

```

[package]
name = "hello-uefi-rs"
version = "0.1.0"
edition = "2024"   
#         ^^^
# https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field  
# 2024 # Latest-As-Of 2026-07-11

# Ray: 包管理, 下载源代码, 并一起编译.
#  (1) 默认 crates.io 网站      | (什么位置都不写) 
#  (2) 或 指定 本地 源代码位置.  | path = the/folder/contains/Cargo.toml
#  (3) 或 指定使用 workspace 里面的定义. | workspace = true
#  (4) ..
#
# 摘要参考: https://doc.rust-lang.org/cargo/guide/dependencies.html
# 详细参考: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
#
[dependencies]

log = "0.4"     # https://crates.io/crates/log  |  0.4.33  # Latest-As-Of 2026-07-11

uefi = {        # https://crates.io/crates/uefi  | 0.38.0  # Latest-As-Of 2026-07-11 
        version = "0.38.0",  
        features = [ "panic_handler", "logger" ]
       }

```