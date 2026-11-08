//!
//! Code is modified from  
//!
//!  https://rust-osdev.github.io/uefi-rs/tutorial/app.html
//!
//!  https://docs.rs/uefi/latest/uefi/
//!
//! Copyright (c) Raymond Yang. 2026-07-11
//! SPDX-License-Identifier: MIT
//!

/*
// https://github.com/codyd51/uefirc/blob/main/src/main_uefi.rs

// https://docs.rs/uefi/latest/uefi/system/fn.with_stdout.html

// https://github.com/sermuns/efimux/blob/main/src/main.rs

// https://blog.hanbings.io/posts/rust-uefi-bootloader/

// https://docs.rs/uefi/latest/uefi/proto/console/text/struct.Output.html

*/

#![no_main]
#![no_std]

// 故意 使用 全名, 以便 搞清楚 从哪里来
//
//use core::time::Duration;
//use log::info;
//use uefi::prelude::*;

// This entry point requires  #[uefi::entry]
//
// 参数被 #[uefi::entry] 吃掉了 ?
//   实际上 整个函数的 签字 整个都被 macro 篡改掉了, 篡改成 uefi 标准入口
//   而且, 在函数开始 注入代码, 把 image_handle, system_table 存了起来
//
// 个人观点：比较厌恶，这种 “自我装逼，朕即天下”，藏掉 公共锚点 的偷鸡摸狗行为
//
#[uefi::entry]
fn main() -> uefi::Status {
    uefi::helpers::init().unwrap();

    let _image_handle = uefi::boot::image_handle();
    let _system_table = uefi::table::system_table_raw();

    //
    // 应该是 uefi-rs 实现了 log 界面, 所以可以通过 log 输出
    //
    log::info!("Hello world! (uefi-rs log)");
    // boot::stall(Duration::from_secs(10));
    uefi::Status::SUCCESS
}
