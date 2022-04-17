use ast::token::*;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("SyntaxError: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },

    #[error("SyntaxError: expected Identifier, found {found:?}")]
    NotFoundIdent { found: TokenKind },
}
