
一键三连

包发布:  https://crates.io/crates/uefi
源代码:  https://github.com/rust-osdev/uefi-rs/ 

文档 (手册，源自 包介绍页): https://docs.rs/uefi/latest/uefi/

文档 (书，源自 代码首页): Rust UEFI Book https://rust-osdev.github.io/uefi-rs/index.html

# https://rust-osdev.github.io/uefi-rs/tutorial/app.html
#
# 
#

```rs

//use core::time::Duration;
// use log::info;
// use uefi::prelude::*;
//use uefi::proto::console::text::Output;

// Ray: 裸奔 原始定义, by rustc

// #[no_mangle]
// pub extern "efiapi" fn efi_main(image_handle: efi::Handle, system_table: *const system::SystemTable) -> efi::Status {
//     unsafe {
//         BOOT_SERVICES = (*system_table).boot_services;
//         RUNTIME_SERVICES = (*system_table).runtime_services;
//         GLOBAL_ALLOCATOR.init(BOOT_SERVICES);
//         init_debug(BOOT_SERVICES);
//     }
// }

// pub fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
//     uefi_services::init(&mut system_table).unwrap();
//     let bs = system_table.boot_services();
//     let bs: &'static BootServices = unsafe {
//         core::mem::transmute(bs)
//     };
// }
```

// https://github.com/codyd51/uefirc/blob/main/src/main_uefi.rs

// https://docs.rs/uefi/latest/uefi/system/fn.with_stdout.html

// https://github.com/sermuns/efimux/blob/main/src/main.rs

// https://blog.hanbings.io/posts/rust-uefi-bootloader/
//

// https://docs.rs/uefi/latest/uefi/proto/console/text/struct.Output.html
