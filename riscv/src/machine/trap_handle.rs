// /// Machine Trap Handling
pub mod mscratch {
    use crate::value_ops;
    value_ops!("mscratch", u64, RW);
}

pub mod mepc {
    use crate::value_ops;
    value_ops!("mepc", u64, RW);
}

pub mod mcause {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    use num_derive::{FromPrimitive, ToPrimitive};

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mcause:u64{
            const code = 0x7FFFFFFFFFFFFFFF;
            const interrupt = 0x1 << 63;
        }
    }

    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum InterruptCause {
        IrqSSoftware = 1,
        IrqMSoftware = 3,
        IrqSTimer = 5,
        IrqMTimer = 7,
        IrqSExternal = 9,
        IrqMExternal = 11,
        IrqCounterOverFlow = 13,
    }

    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum ExceptionCause {
        InstMisaligned = 0,
        InstAccessFault = 1,
        InstIllegal = 2,
        Breakpoint = 3,
        LoadMisaligned = 4,
        LoadAccessFault = 5,
        StoreOrAmoMisaligned = 6,
        StoreOrAmoAccessFault = 7,
        EcallU = 8,
        EcallS = 9,
        EcallM = 11,
        InstPageFault = 12,
        LoadPageFault = 13,
        StoreOrAmoPageFault = 15,
        SoftwareCheck = 18,
        HardwareCheck = 19,
    }
    // pub const PRIORITY:[ExceptionCauseMask;14]=[
    //     ExceptionCauseMask::Breakpoint,
    //     ExceptionCauseMask::InstPageFault,
    //     ExceptionCauseMask::InstAccessFault,
    //     ExceptionCauseMask::InstIllegal,
    //     ExceptionCauseMask::InstMisaligned,
    //     ExceptionCauseMask::EcallU,
    //     ExceptionCauseMask::EcallS,
    //     ExceptionCauseMask::EcallM,
    //     ExceptionCauseMask::LoadMisaligned,
    //     ExceptionCauseMask::StoreOrAmoMisaligned,
    //     ExceptionCauseMask::LoadPageFault,
    //     ExceptionCauseMask::StoreOrAmoPageFault,
    //     ExceptionCauseMask::LoadAccessFault,
    //     ExceptionCauseMask::StoreOrAmoAccessFault,
    // ];
    mask_ops!("mcause", u64, Mcause, RW);
    value_ops!("mcause", u64, RW);
}

pub mod mtval {
    use crate::value_ops;
    value_ops!("mtval", u64, RW);
}

pub mod mip {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mip:u64{
            const ssip = 0x1 << 1;
            const msip = 0x1 << 3;
            const stip = 0x1 << 5;
            const mtip = 0x1 << 7;
            const seip = 0x1 << 9;
            const meip = 0x1 << 11;
            const lcofip = 0x1 << 13;
        }
    }
    mask_ops!("mip", u64, Mip, RW);
    value_ops!("mip", u64, RW);
}

pub mod mtinst {
    use crate::value_ops;
    value_ops!("mtinst", u64, RW);
}

pub mod mtval2 {
    use crate::value_ops;
    value_ops!("mtval2", u64, RW);
}
