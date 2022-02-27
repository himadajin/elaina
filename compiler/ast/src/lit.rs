#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    /// An integer literal: `0`, `1`, `64`
    Int { digits: String },
}
