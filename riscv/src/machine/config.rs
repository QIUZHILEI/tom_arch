pub mod menvcfg {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Menvcfg:u64{
            const fiom = 0x1;
            const cbie = 0x3 << 4;
            const cbcfe = 0x1 << 6;
            const cbze = 0x1 << 7;
            // const pmm = 0x3 << 31;
            const cde = 0x1 << 60;
            const adue = 0x1 << 61;
            const pbmte = 0x1 << 62;
            const stce = 0x1 << 63;
        }
    }
    mask_ops!("menvcfg", u64, Menvcfg, RW);
    value_ops!("menvcfg", u64, RW);
}

pub mod mseccfg {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mseccfg:u64{
            const mml = 0x1;
            const mmwp = 0x1 << 1;
            const rlb = 0x1 << 2;
            const useed = 0x1 << 8;
            const sseed = 0x1 << 9;
            // const pmm = 0x3 << 32;
        }
    }
    mask_ops!("mseccfg", u64, Mseccfg, RW);
    value_ops!("mseccfg", u64, RW);
}
