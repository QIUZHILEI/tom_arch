/// Supervisor Trap Handling

pub mod sscratch {
    use crate::value_ops;
    value_ops!("sscratch", u64, RW);
}

pub mod sepc {
    use crate::value_ops;
    value_ops!("sepc", u64, RW);
}

pub mod scause {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    use num_derive::{FromPrimitive, ToPrimitive};

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Scause:u64{
            const code = 0x7FFFFFFFFFFFFFFF;
            const interrupt = 0x1 << 63;
        }
    }

    #[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
    pub enum InterruptCause {
        IrqSSoftware = 1,
        IrqSTimer = 5,
        IrqSExternal = 9,
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
        InstPageFault = 12,
        LoadPageFault = 13,
        StoreOrAmoPageFault = 15,
        SoftwareCheck = 18,
        HardwareCheck = 19,
    }
    mask_ops!("scause", u64, Scause, RW);
    value_ops!("scause", u64, RW);
}

pub mod stval {
    use crate::value_ops;
    value_ops!("stval", u64, RW);
}

pub mod sip {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Sip:u64{
            const ssip = 0x1 << 1;
            const stip = 0x1 << 5;
            const seip = 0x1 << 9;
            const lcofip = 0x1 << 13;
        }
    }
    mask_ops!("sip", u64, Sip, RW);
    value_ops!("sip", u64, RW);
}

pub mod scountovf {
    use crate::value_ops;
    value_ops!("scountovf", u64, RW);
}
