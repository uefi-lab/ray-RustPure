//!
//!
//! Copyright (c) Raymond Yang. 2026-10-31 to 2026-11-06
//!
//! SPDX-License-Identifier: MIT
//!

#![no_std]
#![no_main]

use patina::char16;
use patina::protocol::ProtocolInterface;
use patina::uefi::boot_services::protocol_handler::HandleSearchType;
use r_efi::efi;

use patina::uefi::boot_services::global_allocator::BootServicesGlobalAllocator; // (2)
use patina::uefi::boot_services::{BootServices, StandardBootServices}; // (1)
// ^^
// Ray: 脑回路清奇的 套娃
// (1) boot_services    需要 Patina 的 "alloc" feature            才能引用 | 因为它 使用了  Rust alloc 特性
// (2) global_allocator 需要 Patina 的 "global_allocator" feature 才能打开 | 因为它 提供了  Rust alloc 需要的 global allocator
//
// 先打开一口缸，缸里有钥匙，钥匙能让缸工作。 螺旋爬楼梯，打个结，让脚下站稳

//====
// Patina 手搓的 Wrapper  (感觉基本就是抄 uefi-rs 脑回路的，一份作业)
//
// sdk/patina/src/uefi/boot_services.rs
//
//   pub struct StandardBootServices {
//       efi_boot_services: Once<*mut efi::BootServices>,
//   }
//   impl BootServices for StandardBootServices {
//
//

//====
// Rust GlobalAlloc 实现 by Patina (这代码组织，基本就是个，花蝴蝶版)
//
// sdk/patina/src/uefi/boot_services/global_allocator.rs
//
// 搭架子
//  (Patina defined) struct BootServicesGlobalAllocator 需要 (Patina defined) trait BootServices 实现
//  (Patina defined) trait BootServices 里面会有 allocate_pool/free_pool 在 UEFI spec 里定义的方法
//
// 干活
//  (1) 为这个 struct 实现 两个自由方法, alloc/dealloc  使用 BootServices 里的 allocate_pool/free_pool
//  (2) 接着把 (Rust defined) trait GlobalAlloc 挂到 (Patina defined) struct 上去
//
//  眼花缭乱，翩翩起舞.
//                         +--------------------+
//                         | trait BootServices |
//                         +--------------------+
//                           ^                ^
//                           | <use>          | <provide>
//                           |                |          +-----------------------------+
//                           |                +----------| struct StandardBootServices |
//                           |                           +-----------------------------+
//                           |
//               +------------------------------------+
//               | struct BootServicesGlobalAllocator |
//               +------------------------------------+
//                              |
//                              |  <provide >   +--------------------+
//                              +-------------->|  trait GlobalAlloc |
//                                              +--------------------+
//

//====
// These two statics can be moved into `fn efi_main()`
//
static BOOT_SERVICES: StandardBootServices = StandardBootServices::new_uninit();

#[global_allocator]
static ALLOCATOR: BootServicesGlobalAllocator<StandardBootServices> = BootServicesGlobalAllocator(&BOOT_SERVICES);

//====
// Patina:
// sdk/patina/src/base/protocol.rs
//
// Ray: 把 r-efi 的调调，弄成 uefi-rs 的调调
//
use efi::protocols::shell::Protocol as Shell;
use efi::protocols::simple_text_output::Protocol as SimpleTextOut;

// //====
// //  similar to uefi-rs
// // 这两个 Wrapper 纯粹是为了捆一个 associated const GUID 上去
// //
//
// struct Shell(pub efi::protocols::shell::Protocol);
// unsafe impl ProtocolInterface for Shell {
//     const PROTOCOL_GUID: patina::BinaryGuid = patina::BinaryGuid::from_string("6302D008-7F9B-4F30-87AC-60C9FEF5DA4E");
// }
//
// struct SimpleTextOut(pub efi::protocols::simple_text_output::Protocol);
// unsafe impl ProtocolInterface for SimpleTextOut {
//     const PROTOCOL_GUID: patina::BinaryGuid = patina::BinaryGuid::from_string("387477C2-69C7-11D2-8E39-00A0C969723B");
// }

#[allow(clippy::not_unsafe_ptr_arg_deref)] // for (*system_table)
#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: efi::Handle, system_table: *mut efi::SystemTable) -> efi::Status {
    //
    // static mut EFI_BS: *mut efi::BootServices = unsafe { (*system_table).boot_services };
    //                                                      ^^^^^^^
    // Ray: Invalid syntax, `static` is compile-time, while the `system_table` is run-time.
    //

    let efi_bs = unsafe { (*system_table).boot_services };
    BOOT_SERVICES.init(efi_bs);

    // Ray:
    //  从现在开始，才可以使用 boot services 的 Wrapper
    //

    let bs = &BOOT_SERVICES; // st.boot_services;

    // Ray:
    //  这里开始，多少有点流氓不讲道理，讲实现
    //
    //  ShellPkg/Application/Shell/Shell.c
    //   UefiMain (
    //     Status = ConsoleLoggerInstall (ShellInfoObject.LogScreenCount, &ShellInfoObject.ConsoleInfo);
    //
    //  ShellPkg/Application/Shell/ConsoleLogger.c
    //   ConsoleLoggerInstall (
    //     Status = gBS->InstallProtocolInterface (&gImageHandle, &gEfiSimpleTextOutProtocolGuid   // Ray: 此处重点
    //     //..
    //     gST->ConsoleOutHandle = gImageHandle;
    //     gST->ConOut           = &(*ConsoleInfo)->OurConOut;
    //     //..
    //
    // 在 EDK2 实现的 Shell 中，会自己实现一份 SimpleTextOut 协议，并替换掉 gST 里的实现
    // 也就是讲，以前例子里对 gST->ConOut 的 SimpleTextOut 访问，其实就是 Shell 的实现
    //
    // 在一个平台上，
    //  - 可能同时存在多个 SimpleTextOut Protocol 实例
    //  - 进入 Shell 以后，通常只有一个 Shell Protocol 实例
    //
    // 所以，和 Shell Protocol 一起伴生的 SimpleTextOut Protocol 实例，才是我们要找的
    //
    // 这种 伴生 关系，是不讲武德的，结合 工具探测 与 查阅源代码 一起印证的
    //
    // -- Raymond, 2026-11-06
    //

    let result = bs.locate_handle_buffer(HandleSearchType::ByProtocol(Shell::PROTOCOL_GUID.as_efi_guid()));
    let handles = match result {
        Ok(handles) => handles,
        Err(err) => {
            // panic!("Failed to locate SimpleTextOut handles");
            return err;
        }
    };

    let shell_handle = handles[0];

    let text_out = unsafe { bs.handle_protocol::<SimpleTextOut>(shell_handle).unwrap() };

    let msg = char16!("Hello, Patina!\r\n");

    unsafe {
        //
        (text_out.output_string)(text_out, msg.as_ptr().cast_mut())
    }

    //efi::Status::SUCCESS
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
