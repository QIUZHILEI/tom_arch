pub mod mnscratch {
    use crate::value_ops;
    value_ops!("mnscratch", u64, RO);
}

pub mod mnepc {
    use crate::value_ops;
    value_ops!("mnepc", u64, RO);
}

pub mod mncause {
    use crate::value_ops;
    value_ops!("mncause", u64, RO);
}

pub mod mnstatus {
    use crate::value_ops;
    value_ops!("mnstatus", u64, RO);
}
