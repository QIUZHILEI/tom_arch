pub mod fflags {
    use crate::value_ops;
    value_ops!("fflags", u64, RW);
}
pub mod frm {
    use crate::value_ops;
    value_ops!("frm", u64, RW);
}
pub mod fcsr {
    use crate::value_ops;
    value_ops!("fcsr", u64, RW);
}
