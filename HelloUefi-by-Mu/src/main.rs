//! ## License
//!
//! Copyright (c) Microsoft Corporation. All rights reserved.
//! SPDX-License-Identifier: BSD-2-Clause-Patent
//!

#![no_std]
#![no_main]
// #![feature(uefi_std)]
#![allow(non_snake_case)]

use r_efi::efi;

// use mu_rust_helpers::function;

// use log::LevelFilter;
// use rust_advanced_logger_dxe::{DEBUG_ERROR, debugln, init_debug};

// use patina::boot_services::protocol_handler::HandleSearchType;

// use patina::boot_services::BootServices as PatinaBootServices;

// gBS : patina::boot_services::BootServices =
// gRS : patina::runtime_services::RuntimeServices
// gST : patina::systemtables::EfiRuntimeServicesTable
// gImageHandle :r_efi::efi::Handel =

// fn create_bs() -> &dyn mu_copy::boot_services::UefiBootServices
// {

// }

mod patina_pick;

// static gBS: Once<RawBootServicess> = Once::new();
// static gImageHandle: Once<*mut efi::Handle> = Once::new();

// static gImageHandle: patina_pick::boot_services::RawBootServices;

// See Also
//  https://github.com/rust-osdev/uefi-rs/blob/main/uefi-std-example/src/main.rs
//
fn setup_common_global(image_handle: efi::Handle, system_table: *const efi::SystemTable) {
    // gImageHandle.call_once(|| image_handle)

    #[path = "mu_copy/mod.rs"]
    mod mu_copy;
    // use rust_advanced_logger_dxe::init_debug;
    use rust_boot_services_allocator_dxe::GLOBAL_ALLOCATOR;

    // // Safety: This block is unsafe because it assumes that system_table and (*system_table).boot_services are correct,
    // // and because it mutates/accesses the global BOOT_SERVICES static.
    unsafe {
        let st = &*system_table; // pointer to reference
        let boot_services = st.boot_services;
        let runtime_services = st.runtime_services;

        mu_copy::SYSTEM_TABLE.store(system_table.cast_mut(), core::sync::atomic::Ordering::SeqCst);
        mu_copy::BOOT_SERVICES.initialize(boot_services);
        mu_copy::RUNTIME_SERVICES.store(runtime_services, core::sync::atomic::Ordering::SeqCst);
        GLOBAL_ALLOCATOR.init(boot_services);
        // init_debug((*system_table).boot_services);

        let wrapper = mu_copy::boot_services::StandardUefiBootServices::new();
        wrapper.initialize(st.boot_services);
        let g_bs: &dyn mu_copy::boot_services::UefiBootServices = &wrapper;

        // use patina::boot_services::StandardBootServices;
        // let gSB = StandardBootServices::new(boot_services);
    }
}

// fn setup_patina_crate(_image_handle: efi::Handle, system_table: *const efi::SystemTable) {
// }

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image_handle: efi::Handle, system_table: *const efi::SystemTable) -> efi::Status {
    setup_common_global(image_handle, system_table);

    // uefi::helpers::init().unwrap();
    // log::info!("UEFI Services Benchmark Test Entry Point");

    efi::Status::SUCCESS
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
