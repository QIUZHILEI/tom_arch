/// Machine Trap Setup

pub mod misa {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    #[allow(warnings)]
    use num_derive::{FromPrimitive, ToPrimitive};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Misa:u64{
            /// Atomic extension
            const A = 0x1;
            /// B extension
            const B = 0x1 << 1;
            /// Compressed extension
            const C = 0x1 << 2;
            /// Double-precision floating-point extension
            const D = 0x1 << 3;
            /// RV32E/64E base ISA
            const E = 0x1 << 4;
            /// Single-precision floating-point extension
            const F = 0x1 << 5;
            /// Hypervisor extension
            const H = 0x1 << 7;
            /// RV32I/64I/128I base ISA
            const I = 0x1 << 8;
            /// Integer Multiply/Divide extension
            const M = 0x1 << 12;
            /// Tentatively reserved for User-Level Interrupts extension
            const N = 0x1 << 13;
            /// Tentatively reserved for Packed-SIMD extension
            const P = 0x1 << 15;
            /// Quad-precision floating-point extension
            const Q = 0x1 << 16;
            /// Supervisor mode implemented
            const S = 0x1 << 18;
            /// User mode implemented
            const U = 0x1 << 20;
            /// Vector extension
            const V = 0x1 << 21;
            /// Non-standard extensions present
            const X = 0x1 << 23;
            const mxl = 0x3 << 62;
        }
    }
    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum Mxl {
        XLEN32 = 1 << 62,
        XLEN64 = 2 << 62,
        XLEN128 = 3 << 62,
    }
    mask_ops!("misa", u64, Misa, RW);
    value_ops!("misa", u64, RW);
}

pub mod mstatus {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    use num_derive::{FromPrimitive, ToPrimitive};

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct MStatus:u64{
            const sie = 0b1 << 1;
            const mie = 0b1 << 3;
            const spie = 0b1 << 5;
            const ube = 0b1 << 6;
            const mpie = 0b1 << 7;
            const spp = 0b1 << 8;
            const vs = 0b11 << 9;
            const mpp = 0b11 << 11;
            const fs = 0b11 << 13;
            const xs = 0b11 << 15;
            const mprv = 0b1 << 17;
            const sum = 0b1 << 18;
            const mxr = 0b1 << 19;
            const tvm = 0b1 << 20;
            const tw = 0b1 << 21;
            const tsr = 0b1 << 22;
            const uxl = 0b11 << 32;
            const sxl = 0b11 << 34;
            const sbe = 0b1 << 36;
            const mbe = 0b1 << 37;
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

    mask_ops!("mstatus", u64, MStatus, RW);
    value_ops!("mstatus", u64, RW);
}

pub mod mtvec {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mtvec:u64{
            const mode = 0b11;
            const base = 0x3FFFFFFFFFFFFFFF << 2;
        }
    }
    mask_ops!("mtvec", u64, Mtvec, RW);
    value_ops!("mtvec", u64, RW);
}

pub mod medeleg {
    use crate::value_ops;
    value_ops!("medeleg", u64, RW);
}

pub mod mideleg {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mideleg:u64{
            const ssi = 0b1 << 1;
            const sti = 0b1 << 5;
            const sei = 0b1 << 9;
        }
    }
    mask_ops!("mideleg", u64, Mideleg, RW);
    value_ops!("mideleg", u64, RW);
}

pub mod mie {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mie:u64{
            const ssie = 0x1 << 1;
            const msie = 0x1 << 3;
            const stie = 0x1 << 5;
            const mtie = 0x1 << 7;
            const seie = 0x1 << 9;
            const meie = 0x1 << 11;
            const lcofie = 0x1 << 13;
        }
    }
    mask_ops!("mie", u64, Mie, RW);
    value_ops!("mie", u64, RW);
}

pub mod mcounteren {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mcounteren:u32{
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
    mask_ops!("mcounteren", u32, Mcounteren, RW);
    value_ops!("mcounteren", u32, RW);
}
