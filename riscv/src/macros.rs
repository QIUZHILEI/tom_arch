#[macro_export]
macro_rules! read_csr {
    ($csr:literal) => {{
        use core::arch::asm;
        let mut res: u64 = 0;
        unsafe {
            asm!(concat!("csrr {0},",$csr),out(reg) res,options(nomem, nostack));
        }
        res
    }};
    ($csr:literal,$ty:ty) => {{
        use core::arch::asm;
        let mut res: $ty = 0;
        unsafe {
            asm!(concat!("csrr {0},",$csr),out(reg) res,options(nomem, nostack));
        }
        res
    }};
}

#[macro_export]
macro_rules! write_csr {
    ($csr:literal,$val:ident) => {{
        use core::arch::asm;
        unsafe{
            asm!(concat!("csrw ",$csr,",{0}"),in(reg) $val,options(nomem, nostack));
        }
    }};
}

#[macro_export]
macro_rules! swap_csr {
    ($csr:literal,$val:ident) => {{
        use core::arch::asm;
        let mut res:u64 = 0;
        unsafe{
            asm!(concat!("csrrw {0},",$csr,",{1}"),out(reg) res,in(reg) $val,options(nomem, nostack));
        }
        res
    }};
    ($csr:literal,$val:ident,$ty:ty) => {{
        use core::arch::asm;
        let mut res:$ty = 0;
        unsafe{
            asm!(concat!("csrrw {0},",$csr,",{1}"),out(reg) res,in(reg) $val,options(nomem, nostack));
        }
        res
    }};
}

#[macro_export]
macro_rules! set_csr {
    ($csr:literal,$val:ident) => {{
        use core::arch::asm;
        unsafe{
            asm!(concat!("csrs ",$csr,",{0}"),in(reg) $val,options(nomem, nostack));
        }
    }};
}

#[macro_export]
macro_rules! clear_csr {
    ($csr:literal,$val:ident) => {{
        use core::arch::asm;
        unsafe{
            asm!(concat!("csrc ",$csr,",{0}"),in(reg) $val,options(nomem, nostack));
        }
    }};
}

#[macro_export]
macro_rules! mask_ops {
    ($csr:literal,$ty:ty,$mask:ty,RO) => {
        #[allow(unused)]
        #[inline]
        pub fn read_mask() -> $mask {
            use crate::read_csr;
            let val = read_csr!($csr, $ty);
            <$mask>::from_bits(val).unwrap()
        }
    };
    ($csr:literal,$ty:ty,$mask:ty,RW) => {
        #[allow(unused)]
        #[inline]
        pub fn read_mask() -> $mask {
            use crate::read_csr;
            let val = read_csr!($csr, $ty);
            <$mask>::from_bits(val).unwrap()
        }
        #[allow(unused)]
        #[inline]
        pub fn write_mask(mask: $mask) {
            use crate::write_csr;
            let val = mask.bits();
            write_csr!($csr, val);
        }
        #[allow(unused)]
        #[inline]
        pub fn swap_mask(mask: $mask) -> $mask {
            use crate::swap_csr;
            let val = mask.bits();
            let pre = swap_csr!($csr, val, $ty);
            <$mask>::from_bits(pre).unwrap()
        }
        #[allow(unused)]
        #[inline]
        pub fn clear_mask(mask: $mask) {
            use crate::clear_csr;
            let val = mask.bits();
            clear_csr!($csr, val);
        }
        #[allow(unused)]
        #[inline]
        pub fn set_mask(mask: $mask) {
            use crate::set_csr;
            let val = mask.bits();
            set_csr!($csr, val);
        }
    };
}

#[macro_export]
macro_rules! value_ops {
    ($csr:literal,$ty:ty,RO) => {
        #[allow(unused)]
        #[inline]
        pub fn read() -> $ty {
            use crate::read_csr;
            read_csr!($csr, $ty)
        }
    };
    ($csr:literal,$ty:ty,RW) => {
        #[allow(unused)]
        #[inline]
        pub fn read() -> $ty {
            use crate::read_csr;
            read_csr!($csr, $ty)
        }
        #[allow(unused)]
        #[inline]
        pub fn write(val: $ty) {
            use crate::write_csr;
            write_csr!($csr, val);
        }
        #[allow(unused)]
        #[inline]
        pub fn swap(val: $ty) -> $ty {
            use crate::swap_csr;
            swap_csr!($csr, val, $ty)
        }
        #[allow(unused)]
        #[inline]
        pub fn clear(val: $ty) {
            use crate::clear_csr;
            clear_csr!($csr, val)
        }
        #[allow(unused)]
        #[inline]
        pub fn set(val: $ty) {
            use crate::set_csr;
            set_csr!($csr, val);
        }
    };
}
