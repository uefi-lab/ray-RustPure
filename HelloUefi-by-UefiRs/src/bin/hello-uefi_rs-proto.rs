//!
//!
//! Copyright (c) Raymond Yang, 2026-11-08
//! SPDX-License-Identifier: MIT
//!

#![no_main]
#![no_std]

use core::mem::MaybeUninit;
use uefi::{
    Handle,
    Identify as _, // for Shell::GUID
    proto::{console::text::Output, shell::Shell},
};

#[uefi::entry]
fn main() -> uefi::Status {
    uefi::helpers::init().unwrap();
    //log::info!("Hello world! (uefi-rs | log)\r\n");

    let _image_handle = uefi::boot::image_handle();
    let _system_table = uefi::table::system_table_raw();

    // Rust 税 - MaybeUninit
    let mut buffer = [MaybeUninit::<Handle>::uninit(); 32];
    let handles = uefi::boot::locate_handle(
        uefi::boot::SearchType::ByProtocol(&Shell::GUID), // 优雅 税 - SearchType
        &mut buffer,
    )
    .expect("Unable to found Shell protocol");

    let handle = handles[0];

    let mut con_out = uefi::boot::open_protocol_exclusive::<Output>(
        handle, //
    )
    .expect("Unable to found SimpleTextOut protocol");

    let msg = uefi::cstr16!("Hello World! (uefi-rs | open_protocol_exclusive)\r\n");

    con_out.output_string(msg).unwrap();

    uefi::Status::SUCCESS
}
