pub mod token;

use crate::token::Token;
use std::str::Chars;

pub struct Lexer<'input> {
    chars: Chars<'input>,
    ch: Option<char>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            ch: None,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.ch = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        match self.ch {
            Some(ch) => {
                if !ch.is_digit(10) {
                    panic!("A non-numeric value was entered")
                }
            }
            None => panic!("Entered string has already reached the end."),
        }

        let mut digits = String::from(self.ch.unwrap());
        loop {
            self.read_char();
            if let Some(c) = self.ch {
                if c.is_digit(10) {
                    digits.push(c);
                    continue;
                }
            }
            break;
        }

        Token::Num(digits)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            Some(ch) => Some(match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,

                '(' => Token::OpenParen,
                ')' => Token::CloseParen,

                '0'..='9' => return Some(self.read_number()),

                _ => unimplemented!(),
            }),
            None => None,
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    macro_rules! test_lexer {
        ($input: expr, $expected: expr) => {
            let mut lexer = Lexer::new($input);
            let mut tokens: Vec<Token> = Vec::new();

            while let Some(token) = lexer.next_token() {
                tokens.push(token);
            }

            assert_eq!($expected, tokens);
        };
    }

    macro_rules! token_num {
        ($value: expr) => {
            Token::Num($value.to_string())
        };
    }

    #[test]
    fn lexer_num() {
        test_lexer!("0", vec![token_num!(0)]);
        test_lexer!("1", vec![token_num!(1)]);
        test_lexer!("16", vec![token_num!(16)]);

        test_lexer!("-16", vec![Token::Minus, token_num!(16)]);
    }

    #[test]
    fn lexer_symbol() {
        test_lexer!("+", vec![Token::Plus]);
        test_lexer!("-", vec![Token::Minus]);
        test_lexer!("*", vec![Token::Star]);
        test_lexer!("/", vec![Token::Slash]);

        test_lexer!("(", vec![Token::OpenParen]);
        test_lexer!(")", vec![Token::CloseParen]);
    }
}
