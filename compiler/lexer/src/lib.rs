mod cursor;
pub mod token;

#[cfg(test)]
mod tests;

use crate::{cursor::*, token::*};

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).advance_token()
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_comsumed();
            Some(cursor.advance_token())
        }
    })
}
