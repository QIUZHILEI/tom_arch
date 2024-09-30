/// Supervisor Trap Setup
pub mod sstatus {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    use num_derive::{FromPrimitive, ToPrimitive};

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Sstatus:u64{
            const sie = 0b1 << 1;
            const spie = 0b1 << 5;
            const ube = 0b1 << 6;
            const spp = 0b1 << 8;
            const vs = 0b11 << 9;
            const fs = 0b11 << 13;
            const xs = 0b11 << 15;
            const sum = 0b1 << 18;
            const mxr = 0b1 << 19;
            const uxl = 0b11 << 32;
            const sd = 0b1 << 63;
        }
    }
    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum VsStatus {
        Off = 0 << 9,
        Initial = 1 << 9,
        Clean = 2 << 9,
        Dirty = 3 << 9,
    }
    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum FsStatus {
        Off = 0 << 13,
        Initial = 1 << 13,
        Clean = 2 << 13,
        Dirty = 3 << 13,
    }
    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum XsStatus {
        AllOff = 0 << 15,
        NoneDirtyOrCleanSomeOn = 1 << 15,
        NoneDirtySomeClean = 2 << 15,
        SomeDirty = 3 << 15,
    }

    mask_ops!("sstatus", u64, Sstatus, RW);
    value_ops!("sstatus", u64, RW);
}

pub mod sie {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Sie:u64{
            const ssie = 0x1 << 1;
            const stie = 0x1 << 5;
            const seie = 0x1 << 9;
            const lcofie = 0x1 << 13;
        }
    }
    mask_ops!("sie", u64, Sie, RW);
    value_ops!("sie", u64, RW);
}

pub mod stvec {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Stvec:u64{
            const mode = 0b11;
            const base = 0x3FFFFFFFFFFFFFFF << 2;
        }
    }
    mask_ops!("stvec", u64, Stvec, RW);
    value_ops!("stvec", u64, RW);
}

pub mod scounteren {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Scounteren:u32{
            const cy = 0x1;
            const tm = 0x1 << 1;
            const ir = 0x1 << 2;
            const hpm3= 0x1 << 3;
            const hpm4= 0x1 << 4;
            const hpm5= 0x1 << 5;
            const hpm6= 0x1 << 6;
            const hpm7= 0x1 << 7;
            const hpm8= 0x1 << 8;
            const hpm9= 0x1 << 9;
            const hpm10 = 0x1 << 10;
            const hpm11 = 0x1 << 11;
            const hpm12 = 0x1 << 12;
            const hpm13 = 0x1 << 13;
            const hpm14 = 0x1 << 14;
            const hpm15 = 0x1 << 15;
            const hpm16 = 0x1 << 16;
            const hpm17 = 0x1 << 17;
            const hpm18 = 0x1 << 18;
            const hpm19 = 0x1 << 19;
            const hpm20 = 0x1 << 20;
            const hpm21 = 0x1 << 21;
            const hpm22 = 0x1 << 22;
            const hpm23 = 0x1 << 23;
            const hpm24 = 0x1 << 24;
            const hpm25 = 0x1 << 25;
            const hpm26 = 0x1 << 26;
            const hpm27 = 0x1 << 27;
            const hpm28 = 0x1 << 28;
            const hpm29 = 0x1 << 29;
            const hpm30 = 0x1 << 30;
            const hpm31 = 0x1 << 31;
        }
    }
    mask_ops!("scounteren", u32, Scounteren, RW);
    value_ops!("scounteren", u32, RW);
}
