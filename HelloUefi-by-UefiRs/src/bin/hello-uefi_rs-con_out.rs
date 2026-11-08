//!
//! See Also
//!  https://rust-osdev.github.io/uefi-rs/how_to/protocols.html
//!
//!  https://github.com/rust-osdev/uefi-rs/blob/main/uefi-std-example/src/main.rs
//!
//! Copyright (c) Raymond Yang. All rights reserved. // 2026-07-11
//! SPDX-License-Identifier: MIT
//!

#![no_main]
#![no_std]

#[uefi::entry]
fn main() -> uefi::Status {
    uefi::helpers::init().unwrap();
    // log::info!("Hello world! (uefi-rs | log)\r\n");

    let _image_handle = uefi::boot::image_handle();
    let _system_table = uefi::table::system_table_raw();

    // 
    let _ = uefi::system::with_stdout(|output| {
        output.output_string(uefi::cstr16!("Hello World! (uefi-rs | lambda)\r\n")) //
    });


    uefi::Status::SUCCESS
}
