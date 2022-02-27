#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    /// An integer literal: `0`, `1`, `64`
    Int { digits: String },

    /// A boolean literal: `true`, `false`
    Bool { value: bool },
}
