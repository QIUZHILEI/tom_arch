pub mod mcountinhibit {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Mcountinhibit:u32{
            const cy = 0x1;
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
    mask_ops!("mcountinhibit", u32, Mcountinhibit, RW);
    value_ops!("mcountinhibit", u32, RW);
}

pub mod mhpmevent3 {
    use crate::value_ops;
    value_ops!("mhpmevent3", u64, RW);
}

pub mod mhpmevent4 {
    use crate::value_ops;
    value_ops!("mhpmevent4", u64, RW);
}

pub mod mhpmevent5 {
    use crate::value_ops;
    value_ops!("mhpmevent5", u64, RW);
}
pub mod mhpmevent6 {
    use crate::value_ops;
    value_ops!("mhpmevent6", u64, RW);
}

pub mod mhpmevent7 {
    use crate::value_ops;
    value_ops!("mhpmevent7", u64, RW);
}

pub mod mhpmevent8 {
    use crate::value_ops;
    value_ops!("mhpmevent8", u64, RW);
}

pub mod mhpmevent9 {
    use crate::value_ops;
    value_ops!("mhpmevent9", u64, RW);
}
pub mod mhpmevent10 {
    use crate::value_ops;
    value_ops!("mhpmevent10", u64, RW);
}

pub mod mhpmevent11 {
    use crate::value_ops;
    value_ops!("mhpmevent11", u64, RW);
}

pub mod mhpmevent12 {
    use crate::value_ops;
    value_ops!("mhpmevent12", u64, RW);
}
pub mod mhpmevent13 {
    use crate::value_ops;
    value_ops!("mhpmevent13", u64, RW);
}
pub mod mhpmevent14 {
    use crate::value_ops;
    value_ops!("mhpmevent14", u64, RW);
}

pub mod mhpmevent15 {
    use crate::value_ops;
    value_ops!("mhpmevent15", u64, RW);
}
pub mod mhpmevent16 {
    use crate::value_ops;
    value_ops!("mhpmevent16", u64, RW);
}
pub mod mhpmevent17 {
    use crate::value_ops;
    value_ops!("mhpmevent17", u64, RW);
}

pub mod mhpmevent18 {
    use crate::value_ops;
    value_ops!("mhpmevent18", u64, RW);
}

pub mod mhpmevent19 {
    use crate::value_ops;
    value_ops!("mhpmevent19", u64, RW);
}

pub mod mhpmevent20 {
    use crate::value_ops;
    value_ops!("mhpmevent20", u64, RW);
}
pub mod mhpmevent21 {
    use crate::value_ops;
    value_ops!("mhpmevent21", u64, RW);
}
pub mod mhpmevent22 {
    use crate::value_ops;
    value_ops!("mhpmevent22", u64, RW);
}

pub mod mhpmevent23 {
    use crate::value_ops;
    value_ops!("mhpmevent23", u64, RW);
}
pub mod mhpmevent24 {
    use crate::value_ops;
    value_ops!("mhpmevent24", u64, RW);
}

pub mod mhpmevent25 {
    use crate::value_ops;
    value_ops!("mhpmevent25", u64, RW);
}

pub mod mhpmevent26 {
    use crate::value_ops;
    value_ops!("mhpmevent26", u64, RW);
}

pub mod mhpmevent27 {
    use crate::value_ops;
    value_ops!("mhpmevent27", u64, RW);
}

pub mod mhpmevent28 {
    use crate::value_ops;
    value_ops!("mhpmevent28", u64, RW);
}

pub mod mhpmevent29 {
    use crate::value_ops;
    value_ops!("mhpmevent29", u64, RW);
}

pub mod mhpmevent30 {
    use crate::value_ops;
    value_ops!("mhpmevent30", u64, RW);
}

pub mod mhpmevent31 {
    use crate::value_ops;
    value_ops!("mhpmevent31", u64, RW);
}
