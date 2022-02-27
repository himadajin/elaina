use crate::lit::*;

pub fn lit_int<T: Into<String>>(digits: T) -> Lit {
    Lit::Int {
        digits: digits.into(),
    }
}
