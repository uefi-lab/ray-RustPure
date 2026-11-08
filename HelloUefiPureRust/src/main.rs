//!
//!
//! Copyright (c) Raymond Yang. 2026-01-15 to 2026-10-31
//! SPDX-License-Identifier: MIT
//!

#![no_std]
#![no_main]

use r_efi::efi;
use r_efi::protocols::simple_text_output::Protocol as SimpleTextOutputProtocol;

// This entry point is 'the raw form' -- default name for linker
//
//  By default, the UEFI targets use the link .. to link binaries
//  (1) into the final PE32+ file suffixed with *.efi
//  (2) The PE subsystem is set to EFI_APPLICATION,  but can be modified by .. /subsystem:
//  (3) Similarly, the entry-point is set to efi_main but can be changed via .. /entry:
//  (4) The panic-strategy is set to abort
//
// 问题: PE32+ 文件入口函数，原型是固定的嘛？
//
//  PE 格式本身只规定： AddressOfEntryPoint = RVA  是个地址，不关心原型
//  加载、执行 PE File 的环境, 确保 参数喂到合适的位置, 符合 ABI 调用
//  这也就是 subsystem 的意义.
//
//  对 UEFI 来说, 也就是 DxeCore 确保, 参数传递到位.
//
//  也就是讲，其实这两个参数，用通用指针，也是可以的，类型其实毫无意义, CPU/ABI 根本不关心
//  这里 真正牛掰的是  efi_main  这个名字, linker 需要用它来干活
//
// -- 2026-07-11
//
// Example: Hello World
// https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html
//
#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: efi::Handle, system_table: *mut efi::SystemTable) -> efi::Status {
    //
    // https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6
    // https://docs.rs/r-efi/latest/r_efi/system/struct.SystemTable.html
    //
    let st: &mut efi::SystemTable = unsafe { &mut *system_table };

    // https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
    // https://docs.rs/r-efi/latest/r_efi/protocols/simple_text_output/index.html
    //
    let stdout: &mut SimpleTextOutputProtocol = unsafe { &mut *(st.con_out as *mut SimpleTextOutputProtocol) };

    #[rustfmt::skip]
    let message: &mut [u16] = &mut [
        'H' as u16,
        'e' as u16,
        'l' as u16,
        'l' as u16,
        'o' as u16,
        ' ' as u16,
        'r' as u16,
        's' as u16,
        '\r' as u16,
        '\n' as u16,
        0,
    ];

    unsafe {
        let _ = (stdout.output_string)(stdout, message.as_mut_ptr());
    }

    efi::Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
