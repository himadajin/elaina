use ast::token::{KwKind, Token};
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

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_int(&mut self) -> Token {
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

        Token::Integer(digits)
    }

    fn read_str(&mut self) -> String {
        let is_letter = |c: char| c.is_ascii_alphanumeric() || c == '_';

        let ch = self.ch.expect("error: tried to process an empty string");
        if !is_letter(ch) {
            panic!("error: tried to process non-alphanumeric character");
        }

        let mut literal = String::from(ch);
        loop {
            self.read_char();
            match self.ch {
                Some(ch) => {
                    if is_letter(ch) {
                        literal.push(ch);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        literal
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            Some(ch) => Some(match ch {
                '=' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::EqEq
                    }
                    _ => Token::Eq,
                },
                '<' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::Le
                    }
                    _ => Token::Lt,
                },
                '>' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::Ge
                    }
                    _ => Token::Gt,
                },

                '!' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::Ne
                    }
                    _ => todo!(),
                },

                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,

                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,

                ';' => Token::Semi,
                ':' => Token::Colon,

                '0'..='9' => return Some(self.read_int()),

                _ => {
                    let literal = self.read_str();
                    let token = match literal.as_str() {
                        "let" => Token::Keyword(KwKind::Let),
                        "true" => Token::Keyword(KwKind::True),
                        "false" => Token::Keyword(KwKind::False),
                        "println" => Token::Keyword(KwKind::Println),

                        _ => Token::Ident(literal),
                    };

                    return Some(token);
                }
            }),
            None => None,
        };

        self.read_char();

        token
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub fn run_lexer(input: &str) -> Vec<Token> {
    Lexer::new(input).into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    macro_rules! test_lexer {
        ($input: expr, $expected: expr) => {
            let tokens = run_lexer($input);

            assert_eq!($expected, tokens);
        };
    }

    macro_rules! token_int {
        ($value: expr) => {
            Token::Integer($value.to_string())
        };
    }

    macro_rules! token_ident {
        ($value: expr) => {
            Token::Ident($value.to_string())
        };
    }

    macro_rules! token_kw {
        ($value: expr) => {
            Token::Keyword($value)
        };
    }

    #[test]
    fn lexer_num() {
        test_lexer!("0", vec![token_int!(0)]);
        test_lexer!("1", vec![token_int!(1)]);
        test_lexer!("16", vec![token_int!(16)]);

        test_lexer!("-16", vec![Token::Minus, token_int!(16)]);
    }

    #[test]
    fn lexer_symbol() {
        test_lexer!("=", vec![Token::Eq]);

        test_lexer!("<", vec![Token::Lt]);
        test_lexer!("<=", vec![Token::Le]);
        test_lexer!("==", vec![Token::EqEq]);
        test_lexer!("!=", vec![Token::Ne]);
        test_lexer!(">=", vec![Token::Ge]);
        test_lexer!(">", vec![Token::Gt]);

        test_lexer!("+", vec![Token::Plus]);
        test_lexer!("-", vec![Token::Minus]);
        test_lexer!("*", vec![Token::Star]);
        test_lexer!("/", vec![Token::Slash]);

        test_lexer!("(", vec![Token::OpenParen]);
        test_lexer!(")", vec![Token::CloseParen]);

        test_lexer!("{", vec![Token::OpenBrace]);
        test_lexer!("}", vec![Token::CloseBrace]);

        test_lexer!(";", vec![Token::Semi]);
        test_lexer!(":", vec![Token::Colon]);
    }

    #[test]
    fn lexer_ident() {
        test_lexer!("foo", vec![token_ident!("foo")]);
        test_lexer!("foo bar", vec![token_ident!("foo"), token_ident!("bar")]);
        test_lexer!("1 foo", vec![token_int!(1), token_ident!("foo")]);
        test_lexer!("foo 1", vec![token_ident!("foo"), token_int!(1)]);
    }

    #[test]
    fn lexer_keyword() {
        test_lexer!("let", vec![token_kw!(KwKind::Let)]);
        test_lexer!("true", vec![token_kw!(KwKind::True)]);
        test_lexer!("false", vec![token_kw!(KwKind::False)]);

        test_lexer!(
            "let a: i32 = 0;",
            vec![
                token_kw!(KwKind::Let),
                token_ident!("a"),
                Token::Colon,
                token_ident!("i32"),
                Token::Eq,
                token_int!(0),
                Token::Semi
            ]
        );
        test_lexer!("leta", vec![token_ident!("leta")]);
    }

    #[test]
    fn lexer_expr() {
        test_lexer!(
            "(1)",
            vec![Token::OpenParen, token_int!(1), Token::CloseParen]
        );
        test_lexer!(
            "(a)",
            vec![Token::OpenParen, token_ident!("a"), Token::CloseParen]
        );
    }
}
