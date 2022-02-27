use crate::lit::*;

pub fn lit_int<T: Into<String>>(digits: T) -> Lit {
    Lit::Int {
        digits: digits.into(),
    }
}

pub fn lit_bool(value: bool) -> Lit {
    Lit::Bool { value: value }
}
