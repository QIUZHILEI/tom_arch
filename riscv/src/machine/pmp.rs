use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PmpConfig:u8{
        const r = 0x1;
        const w = 0x1 << 1;
        const x = 0x1 << 2;
        const a = 0x3 << 3;
        const l = 0x1 << 7;
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PmpAddrType {
    Off = 0 << 3,
    Tor = 1 << 3,
    Na4 = 2 << 3,
    NaPot = 3 << 3,
}

pub mod pmpcfg0 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg0:u64{
            const cfg0 = 0xFF;
            const cfg1 = 0xFF << 8;
            const cfg2 = 0xFF << 16;
            const cfg3 = 0xFF << 24;
            const cfg4 = 0xFF << 32;
            const cfg5 = 0xFF << 40;
            const cfg6 = 0xFF << 48;
            const cfg7 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg0", u64, Pmpcfg0, RW);
    value_ops!("pmpcfg0", u64, RW);
}

pub mod pmpcfg2 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg2:u64{
            const cfg8 = 0xFF;
            const cfg9 = 0xFF << 8;
            const cfg10 = 0xFF << 16;
            const cfg11 = 0xFF << 24;
            const cfg12 = 0xFF << 32;
            const cfg13 = 0xFF << 40;
            const cfg14 = 0xFF << 48;
            const cfg15 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg2", u64, Pmpcfg2, RW);
    value_ops!("pmpcfg2", u64, RW);
}

pub mod pmpcfg4 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg4:u64{
            const cfg16 = 0xFF;
            const cfg17 = 0xFF << 8;
            const cfg18 = 0xFF << 16;
            const cfg19 = 0xFF << 24;
            const cfg20 = 0xFF << 32;
            const cfg21 = 0xFF << 40;
            const cfg22 = 0xFF << 48;
            const cfg23 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg4", u64, Pmpcfg4, RW);
    value_ops!("pmpcfg4", u64, RW);
}

pub mod pmpcfg6 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg6:u64{
            const cfg24 = 0xFF;
            const cfg25 = 0xFF << 8;
            const cfg26 = 0xFF << 16;
            const cfg27 = 0xFF << 24;
            const cfg28 = 0xFF << 32;
            const cfg29 = 0xFF << 40;
            const cfg30 = 0xFF << 48;
            const cfg31 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg6", u64, Pmpcfg6, RW);
    value_ops!("pmpcfg6", u64, RW);
}

pub mod pmpcfg8 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg8:u64{
            const cfg32 = 0xFF;
            const cfg33 = 0xFF << 8;
            const cfg34 = 0xFF << 16;
            const cfg35 = 0xFF << 24;
            const cfg36 = 0xFF << 32;
            const cfg37 = 0xFF << 40;
            const cfg38 = 0xFF << 48;
            const cfg39 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg8", u64, Pmpcfg8, RW);
    value_ops!("pmpcfg8", u64, RW);
}

pub mod pmpcfg10 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg10:u64{
            const cfg40 = 0xFF;
            const cfg41 = 0xFF << 8;
            const cfg42 = 0xFF << 16;
            const cfg43 = 0xFF << 24;
            const cfg44 = 0xFF << 32;
            const cfg45 = 0xFF << 40;
            const cfg46 = 0xFF << 48;
            const cfg47 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg10", u64, Pmpcfg10, RW);
    value_ops!("pmpcfg10", u64, RW);
}

pub mod pmpcfg12 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg12:u64{
            const cfg48 = 0xFF;
            const cfg49 = 0xFF << 8;
            const cfg50 = 0xFF << 16;
            const cfg51 = 0xFF << 24;
            const cfg52 = 0xFF << 32;
            const cfg53 = 0xFF << 40;
            const cfg54 = 0xFF << 48;
            const cfg55 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg12", u64, Pmpcfg12, RW);
    value_ops!("pmpcfg12", u64, RW);
}

pub mod pmpcfg14 {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Pmpcfg14:u64{
            const cfg56 = 0xFF;
            const cfg57 = 0xFF << 8;
            const cfg58 = 0xFF << 16;
            const cfg59 = 0xFF << 24;
            const cfg60 = 0xFF << 32;
            const cfg61 = 0xFF << 40;
            const cfg62 = 0xFF << 48;
            const cfg63 = 0xFF << 56;
        }
    }
    mask_ops!("pmpcfg14", u64, Pmpcfg14, RW);
    value_ops!("pmpcfg14", u64, RW);
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PmpAddr:u64{
        const addr = 0x3FFFFFFFFFFFFF;
    }
}


pub mod pmpaddr0 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr0", u64, PmpAddr, RW);
}

pub mod pmpaddr1 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr1", u64, PmpAddr, RW);
}

pub mod pmpaddr2 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr2", u64, PmpAddr, RW);
}

pub mod pmpaddr3 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr3", u64, PmpAddr, RW);
}

pub mod pmpaddr4 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr4", u64, PmpAddr, RW);
}

pub mod pmpaddr5 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr5", u64, PmpAddr, RW);
}

pub mod pmpaddr6 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr6", u64, PmpAddr, RW);
}

pub mod pmpaddr7 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr7", u64, PmpAddr, RW);
}

pub mod pmpaddr8 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr8", u64, PmpAddr, RW);
}

pub mod pmpaddr9 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr9", u64, PmpAddr, RW);
}

pub mod pmpaddr10 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr10", u64, PmpAddr, RW);
}

pub mod pmpaddr11 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr11", u64, PmpAddr, RW);
}

pub mod pmpaddr12 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr12", u64, PmpAddr, RW);
}

pub mod pmpaddr13 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr13", u64, PmpAddr, RW);
}

pub mod pmpaddr14 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr14", u64, PmpAddr, RW);
}

pub mod pmpaddr15 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr15", u64, PmpAddr, RW);
}

pub mod pmpaddr16 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr16", u64, PmpAddr, RW);
}

pub mod pmpaddr17 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr17", u64, PmpAddr, RW);
}

pub mod pmpaddr18 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr18", u64, PmpAddr, RW);
}

pub mod pmpaddr19 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr19", u64, PmpAddr, RW);
}

pub mod pmpaddr20 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr20", u64, PmpAddr, RW);
}

pub mod pmpaddr21 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr21", u64, PmpAddr, RW);
}

pub mod pmpaddr22 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr22", u64, PmpAddr, RW);
}

pub mod pmpaddr23 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr23", u64, PmpAddr, RW);
}

pub mod pmpaddr24 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr24", u64, PmpAddr, RW);
}

pub mod pmpaddr25 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr25", u64, PmpAddr, RW);
}

pub mod pmpaddr26 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr26", u64, PmpAddr, RW);
}

pub mod pmpaddr27 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr27", u64, PmpAddr, RW);
}

pub mod pmpaddr28 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr28", u64, PmpAddr, RW);
}

pub mod pmpaddr29 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr29", u64, PmpAddr, RW);
}

pub mod pmpaddr30 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr30", u64, PmpAddr, RW);
}

pub mod pmpaddr31 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr31", u64, PmpAddr, RW);
}

pub mod pmpaddr32 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr32", u64, PmpAddr, RW);
}

pub mod pmpaddr33 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr33", u64, PmpAddr, RW);
}

pub mod pmpaddr34 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr34", u64, PmpAddr, RW);
}

pub mod pmpaddr35 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr35", u64, PmpAddr, RW);
}

pub mod pmpaddr36 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr36", u64, PmpAddr, RW);
}

pub mod pmpaddr37 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr37", u64, PmpAddr, RW);
}

pub mod pmpaddr38 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr38", u64, PmpAddr, RW);
}

pub mod pmpaddr39 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr39", u64, PmpAddr, RW);
}

pub mod pmpaddr40 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr40", u64, PmpAddr, RW);
}

pub mod pmpaddr41 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr41", u64, PmpAddr, RW);
}

pub mod pmpaddr42 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr42", u64, PmpAddr, RW);
}

pub mod pmpaddr43 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr43", u64, PmpAddr, RW);
}

pub mod pmpaddr44 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr44", u64, PmpAddr, RW);
}

pub mod pmpaddr45 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr45", u64, PmpAddr, RW);
}

pub mod pmpaddr46 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr46", u64, PmpAddr, RW);
}

pub mod pmpaddr47 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr47", u64, PmpAddr, RW);
}

pub mod pmpaddr48 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr48", u64, PmpAddr, RW);
}
pub mod pmpaddr49 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr49", u64, PmpAddr, RW);
}

pub mod pmpaddr50 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr50", u64, PmpAddr, RW);
}

pub mod pmpaddr51 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr51", u64, PmpAddr, RW);
}

pub mod pmpaddr52 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr52", u64, PmpAddr, RW);
}

pub mod pmpaddr53 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr53", u64, PmpAddr, RW);
}

pub mod pmpaddr54 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr54", u64, PmpAddr, RW);
}

pub mod pmpaddr55 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr55", u64, PmpAddr, RW);
}

pub mod pmpaddr56 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr56", u64, PmpAddr, RW);
}

pub mod pmpaddr57 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr57", u64, PmpAddr, RW);
}

pub mod pmpaddr58 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr58", u64, PmpAddr, RW);
}

pub mod pmpaddr59 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr59", u64, PmpAddr, RW);
}

pub mod pmpaddr60 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr60", u64, PmpAddr, RW);
}

pub mod pmpaddr61 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr61", u64, PmpAddr, RW);
}

pub mod pmpaddr62 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr62", u64, PmpAddr, RW);
}

pub mod pmpaddr63 {
    use super::PmpAddr;
    use crate::mask_ops;
    mask_ops!("pmpaddr63", u64, PmpAddr, RW);
}
