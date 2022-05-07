use span::{Span, Symbol};

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum NameResolutionError {
    #[error("error: unresolved name {name:?} at ({span:?}) was used")]
    UnresolvedNameUsed { name: Symbol, span: Span },
}
