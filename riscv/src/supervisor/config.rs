/// Supervisor Configuration
pub mod senvcfg {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;

    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Senvcfg:u64{
            const fiom = 0x1;
            const cbie = 0x3 << 4;
            const cbcfe = 0x1 << 6;
            const cbze = 0x1 << 7;
            // const pmm = 0x3 << 31;
        }
    }
    mask_ops!("senvcfg", u64, Senvcfg, RW);
    value_ops!("senvcfg", u64, RW);
}
