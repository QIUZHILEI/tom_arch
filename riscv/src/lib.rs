#![no_std]
#[cfg(feature = "D")]
pub mod debug;
#[cfg(feature = "H")]
pub mod hypervisor;

#[cfg(feature = "M")]
pub mod machine;
#[cfg(feature = "S")]
pub mod supervisor;
#[cfg(feature = "U")]
pub mod user;
#[cfg(feature = "V")]
pub mod virt;

#[cfg(any(feature = "S", feature = "M", feature = "U"))]
mod asm;
#[cfg(any(
    feature = "S",
    feature = "M",
    feature = "D",
    feature = "H",
    feature = "V"
))]
#[cfg(any(feature = "S", feature = "M", feature = "U"))]
pub use asm::*;
#[cfg(any(feature = "S", feature = "M", feature = "U"))]
mod macros;
#[cfg(feature = "timer")]
mod mmap_time;
#[cfg(feature = "timer")]
pub use mmap_time::*;

#[cfg(feature = "D")]
pub use debug::*;
#[cfg(feature = "H")]
pub use hypervisor::*;
#[cfg(feature = "M")]
pub use machine::*;
#[cfg(feature = "S")]
pub use supervisor::*;
#[cfg(feature = "U")]
pub use user::*;
#[cfg(feature = "V")]
pub use virt::*;

pub const MODE_U: u64 = 0;
pub const MODE_S: u64 = 0x01;
pub const MODE_M: u64 = 0x11;
