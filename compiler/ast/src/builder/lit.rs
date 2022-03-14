use crate::lit::*;
use span::span::DUMMY_SP;

pub fn lit_int(value: u128) -> Lit {
    Lit {
        kind: LitKind::Int(value),
        span: DUMMY_SP,
    }
}

pub fn lit_bool(value: bool) -> Lit {
    Lit {
        kind: LitKind::Bool(value),
        span: DUMMY_SP,
    }
}
