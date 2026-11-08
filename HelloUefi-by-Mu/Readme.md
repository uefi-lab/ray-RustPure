

一键三连
包:   https://crates.io/crates/mu_rust_helpers
代码: https://github.com/microsoft/mu_rust_helpers
文档: 


https://github.com/OpenDevicePartnership/patina-apps/blob/main/Makefile.toml


Common/MU/HidPkg/UefiHidDxeV2/UefiHidDxeV2.inf
  混合型.  纯血 Rust 代码(.EFI),  混合 EDK2 打包

# Ray: 使用 r-efi 定义一个 Protocol
#
# MU | HidPkg.dec | gHidIoProtocolGuid 的 Rust 平行 对等
#
Common/MU/HidPkg/Crates/HidIo/Cargo.toml
  纯血型.  纯血 Rust 代码(.rlib)
  
# Ray:
#
# EDK2 | MdeModulePkg.dec | gUsbKeyboardLayoutKeyGuid
#
Common/MU/HidPkg/Crates/HiiKeyboardLayout/Cargo.toml
  纯血型.  纯血 Rust 代码(.rlib)

Common/MU/AdvLoggerPkg/Crates/RustAdvancedLoggerDxe/Cargo.toml
  纯血型.  纯血 Rust 代码(.rlib)

Common/MU/MsCorePkg/Crates/RustBootServicesAllocatorDxe/Cargo.toml
  纯血型.  纯血 Rust 代码(.rlib)

Common/MU/MsWheaPkg/Crates/MuTelemetryHelperLib/Cargo.toml  
  纯血型.  纯血 Rust 代码(.rlib)

Rust 本身支持：

| 类型           | UEFI 里是否常用 | 说明                                 |
| ------------ | ---------- | ---------------------------------- |
| `rlib`       | ✅          | Rust 原生库，供其它 Rust crate 依赖         |
| `staticlib`  | ✅          | 生成 `.lib`（MSVC）或 `.a`，最常用于 EDK2 集成 |
| `cdylib`     | ❌          | C ABI 动态库                          |
| `dylib`      | ❌          | Rust ABI 动态库                       |
| `proc-macro` | 构建机用       | 编译期宏，不会进入固件                        |

对于 x86_64-unknown-uefi target，通常只有 .efi、rlib、staticlib 有意义；
dylib / cdylib 不是“生成了没人加载”，而是 target 本身通常就不允许生成。



