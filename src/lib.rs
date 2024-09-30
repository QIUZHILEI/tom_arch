#![no_std]
#[cfg(target_arch = "aarch64")]
pub extern crate arm;
#[cfg(target_arch = "riscv64")]
pub extern crate riscv;
