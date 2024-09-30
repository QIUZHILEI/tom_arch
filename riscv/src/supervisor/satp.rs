pub enum Mode {
    // No translation or protection
    Bare = 0,
    Sv39 = 0x8,
    Sv48 = 0x9,
    Sv57 = 0xa,
    Sv64 = 0xb,
}

/// RISC-V Linux Kernel SV39
/// ------------------------
///
///   ========================================================================================================================
///       Start addr    |   Offset   |     End addr     |  Size   | VM area description
///   ========================================================================================================================
///                     |            |                  |         |
///    0000000000000000 |    0       | 0000003fffffffff |  256 GB | user-space virtual memory, different per mm
///   __________________|____________|__________________|_________|___________________________________________________________
///                     |            |                  |         |
///    0000004000000000 | +256    GB | ffffffbfffffffff | ~16M TB | ... huge, almost 64 bits wide hole of non-canonical
///                     |            |                  |         |     virtual memory addresses up to the -256 GB
///                     |            |                  |         |     starting offset of kernel mappings.
///   __________________|____________|__________________|_________|___________________________________________________________
///                                                               |
///                                                               | Kernel-space virtual memory, shared between all processes:
///   ____________________________________________________________|___________________________________________________________
///                     |            |                  |         |
///    ffffffc6fea00000 | -228    GB | ffffffc6feffffff |    6 MB | fixmap
///    ffffffc6ff000000 | -228    GB | ffffffc6ffffffff |   16 MB | PCI io
///    ffffffc700000000 | -228    GB | ffffffc7ffffffff |    4 GB | vmemmap
///    ffffffc800000000 | -224    GB | ffffffd7ffffffff |   64 GB | vmalloc/ioremap space
///    ffffffd800000000 | -160    GB | fffffff6ffffffff |  124 GB | direct mapping of all physical memory
///    fffffff700000000 |  -36    GB | fffffffeffffffff |   32 GB | kasan
///   __________________|____________|__________________|_________|____________________________________________________________
///                                                               |
///                                                               |
///   ____________________________________________________________|____________________________________________________________
///                     |            |                  |         |
///    ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | modules, BPF
///    ffffffff80000000 |   -2    GB | ffffffffffffffff |    2 GB | kernel
///   __________________|____________|__________________|_________|____________________________________________________________
pub const ASIDMAX: u8 = 16;
pub mod sv39 {
    use core::mem;

    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};

    bitflags! {
        #[derive(Debug,Clone, Copy)]
        pub struct Satp:u64{
            const ppn = 0xFFFFFFFFFFF;
            const asid = 0x3F << 44;
            const mode = 0xF << 60;
        }
    }
    mask_ops!("satp", u64, Satp, RW);
    value_ops!("satp", u64, RW);
    pub type Pte = u64;
    pub type PageTable = *mut u64;
    pub const MODE: u64 = (super::Mode::Sv39 as u64) << 60;
    pub const PG_LEVEL: u64 = 3;
    pub const PG_OFFSET: u64 = 12;
    pub const PN_OFFSET: u64 = 9;
    pub const PG_SIZE: u64 = 4096;
    pub const PTE_SIZE: u64 = mem::size_of::<Pte>() as u64;
    pub const PER_PG_PTE_NUM: u64 = PG_SIZE / PTE_SIZE;
    pub const MAX_VA: u64 = 1 << (PN_OFFSET * PG_LEVEL + PG_OFFSET - 1);

    pub const PTE_V: u64 = 0;
    pub const PTE_R: u64 = 0b1 << 1;
    pub const PTE_W: u64 = 0b1 << 2;
    pub const PTE_X: u64 = 0b1 << 3;
    pub const PTE_U: u64 = 0b1 << 4;
    pub const PTE_G: u64 = 0b1 << 5;
    pub const PTE_A: u64 = 0b1 << 6;
    pub const PTE_D: u64 = 0b1 << 7;
    pub const PTE_RSW: u64 = 8;
    pub const PTE_PPN0: u64 = 10;
    pub const PTE_PPN1: u64 = 19;
    pub const PTE_PPN2: u64 = 28;
    pub const PTE_PPN3: u64 = 61;
    pub const PTE_N: u64 = 63;

    pub const VM_USER_SPACE_START: u64 = 0x0;
    pub const VM_USER_SPACE_SIZE: u64 = 256 * 1024 * 1024 * 1024;

    pub const VM_FIX_MAP_START: u64 = 0xffffffc6fea00000;
    pub const VM_FIX_MAP_SIZE: u64 = 6 * 1024 * 1024;

    pub const VM_PCI_IO_START: u64 = 0xffffffc6ff000000;
    pub const VM_PCI_IO_SIZE: u64 = 16 * 1024 * 1024;

    pub const VM_MEM_MAP_START: u64 = 0xffffffc700000000;
    pub const VM_MEM_MAP_SIZE: u64 = 4 * 1024 * 1024 * 1024;

    pub const VM_ALLOC_START: u64 = 0xffffffc800000000;
    pub const VM_ALLOC_SIZE: u64 = 64 * 1024 * 1024 * 1024;

    pub const VM_DIRECT_MAP_START: u64 = 0xffffffd800000000;
    pub const VM_DIRECT_MAP_SIZE: u64 = 124 * 1024 * 1024 * 1024;

    pub const VM_KA_SCAN_START: u64 = 0xfffffff700000000;
    pub const VM_KA_SCAN_SIZE: u64 = 32 * 1024 * 1024 * 1024;

    pub const VM_BPF_START: u64 = 0xffffffff00000000;
    pub const VM_BPF_SIZE: u64 = 2 * 1024 * 1024 * 1024;

    pub const VM_KERNEL_START: u64 = 0xffffffff80000000;
    pub const VM_KERNEL_SIZE: u64 = 2 * 1024 * 1024 * 1024;

    #[inline]
    pub const fn pa2pte(pa: u64) -> u64 {
        (pa >> 12) << 10
    }
    #[inline]
    pub const fn pte2pa(pa: u64) -> u64 {
        (pa >> 10) << 12
    }
    #[inline]
    pub const fn pte_flags(pte: u64) -> u64 {
        pte & 0x3FF
    }
    #[inline]
    pub const fn pg_round_up(size: u64) -> u64 {
        (size + PG_SIZE - 1) & !(PG_SIZE - 1)
    }
    #[inline]
    pub const fn pg_round_down(addr: u64) -> u64 {
        addr & !(PG_SIZE - 1)
    }

    #[inline]
    pub const fn pn_shift(level: u64) -> u64 {
        PG_OFFSET + PN_OFFSET * level
    }

    #[inline]
    pub const fn pn(level: u64, va: u64) -> u64 {
        (va >> pn_shift(level)) & 0x1FF
    }

    #[inline]
    pub fn make_satp(pt: PageTable) -> u64 {
        MODE | (pt as u64 >> PG_OFFSET)
    }
}

pub mod sv48 {}

pub mod sv57 {}

pub mod sv64 {}
