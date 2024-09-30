pub mod scountinhibit {
    use crate::{mask_ops, value_ops};
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Scountinhibit:u32{
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
    mask_ops!("scountinhibit", u32, Scountinhibit, RW);
    value_ops!("scountinhibit", u32, RW);
}
