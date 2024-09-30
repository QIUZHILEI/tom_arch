// /// Machine Information Registers
pub mod mvendorid {
    use bitflags::bitflags;

    use crate::{mask_ops, value_ops};
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mvendorid:u32{
            const offset = 0x7F;
            const bank = 0x1FFFFFF << 7;
        }
    }
    mask_ops!("mvendorid", u32, Mvendorid, RO);
    value_ops!("mvendorid", u32, RO);
}

pub mod marchid {
    use crate::value_ops;
    value_ops!("marchid", u64, RO);
}

pub mod mimpid {
    use crate::value_ops;
    value_ops!("mimpid", u64, RO);
}

pub mod mhartid {
    use crate::value_ops;
    value_ops!("mhartid", u64, RO);
}

pub mod mconfigptr {
    use crate::value_ops;
    value_ops!("mconfigptr", u64, RO);
}
