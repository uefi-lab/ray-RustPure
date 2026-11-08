//!
//!
//! https://doc.rust-lang.org/rustc/platform-support/unknown-uefi.html#example-hello-world-with-std
//!
//! Copyright (c) Raymond Yang. 2026
//! SPDX-License-Identifier: MIT
//!

#![feature(uefi_std)]

use r_efi::efi;
use std::os::uefi::ffi::OsStrExt;

// This entry point requires Rust  #![feature(uefi_std)]
//
fn main() {
    //
    // 入口参数 image_handle , system_table
    // 被 uefi_std 映射到 (复制到) env 中,
    // 都是 通用指针类型
    //
    let system_table = std::os::uefi::env::system_table();
    let _image_handle = std::os::uefi::env::image_handle();

    // Ray:
    //   可使用常见 rust 功能, 透过 uefi_std 实现
    //   实现细节……
    //
    std::println!("Hello World! (uefi_std | println)\r\n");

    //
    // Use System Table Directly
    //

    // Casting
    let st = system_table.as_ptr() as *mut efi::SystemTable;
    let s: Vec<u16> = std::ffi::OsString::from("Hello World! (uefi_std | by proto)\r\n\0").encode_wide().collect();
    let r = unsafe {
        // Explit type copy
        // let con_out: *mut simple_text_output::Protocol = (*st).con_out;
        // let output_string: extern "efiapi" fn(_: *mut simple_text_output::Protocol, *mut u16) -> efi::Status = (*con_out).output_string;
        let con_out = (*st).con_out;
        let output_string = (*con_out).output_string;
        output_string(con_out, s.as_ptr() as *mut efi::Char16)
    };
    assert!(!r.is_error())
}
