
用了两三天, 用 uefi_std 写了一两个程序

决定：主流不用  #![feature(uefi_std)]

不是说 Rust 的这个 语言特性不好, 

  只是说 uefi_std 不适合, 三观以 UEFI 为中心建立的 从业人员

  而是更适合, 三观以 Rust 为中心建立的 app 开发人员


不是说不玩 uefi_std 了, 只是说玩它要单开一脉, 因为

 (1) 三观不同, 脑回路 锚定在 Rust 开发 app 的三观基础 上  
 (2) cargo workspace 不支持 混合使用  
 (3) rust-analyzer  貌似, 也不支持 混合使用  
 (4) 居然 println!() 换行是错的, .. 他妈的, 你让我怎么用？到处加补丁嘛?  

这个 uefi_std 依然有趣, 但玩的脑回路不同, 与 UEFI/EDK2 里的三观不同.  

但上手真的快, println!() 用起来真顺手.  


-- Raymond, 2026-07-13

