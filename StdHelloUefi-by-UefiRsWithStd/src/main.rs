//!
//! // https://github.com/rust-osdev/uefi-rs/blob/main/uefi-std-example/src/main.rs
//!
//! Copyright (c) Raymond Yang. All rights reserved.
//! SPDX-License-Identifier: MIT
//!

/*
 https://rust-osdev.github.io/uefi-rs/tutorial/app.html
 https://docs.rs/uefi/latest/uefi/
*/

// Last-checked 2026-07-11
//
// Note: In Rust 1.82.0-nightly and before, the `uefi_std` feature is
// required for accessing `std::os::uefi::env::*`. The other default
// functionality doesn't need a nightly toolchain (with Rust 1.80 and later),
// but with that limited functionality you - currently - also can't integrate
// the `uefi` crate.
#![feature(uefi_std)]

/// Performs the necessary setup code for the `uefi` crate.
fn setup_uefi_crate() {
    //
    // Get from (rust) `uefi_std` feature
    //
    let st = std::os::uefi::env::system_table();
    let ih = std::os::uefi::env::image_handle();

    // Save `image_handle` and `system_table` to (uefi-rs) `uefi` crate
    //
    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());

        let ih = uefi::Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }

    // Get from (uefi-rs) `uefi` crate
    //
    let _rs_system_table = uefi::table::system_table_raw();
    let _rs_image_handle = uefi::boot::image_handle();
}

fn main() {
    println!("Hello World (uefi_std | println)");
    //     ^^^
    // newline bug in Rust println!
    println!("New line\r");

    setup_uefi_crate();
    println!("UEFI-Version is {} (from uefi-rs)\r\n", uefi::system::uefi_revision());

    // use uefi::runtime::ResetType;
    // use uefi::{ Status};
    // uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
