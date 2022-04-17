use crate::span::{Span, DUMMY_SP};

use derive_more::{From, Into};
use typed_index_collections::TiVec;

use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, From, Into, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Symbol(usize);

impl Symbol {
    #[inline]
    pub const fn new(idx: usize) -> Self {
        Symbol(idx)
    }

    #[inline]
    pub const fn ident_nth(n: usize) -> Self {
        Symbol(KEYWORDS.len() + n)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    #[inline]
    pub fn new<S: Into<Symbol>>(name: S, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    #[inline]
    pub fn with_dummy_span<S: Into<Symbol>>(name: S) -> Self {
        Self {
            name: name.into(),
            span: DUMMY_SP,
        }
    }
}

pub struct SymbolMap<'a> {
    names: HashMap<&'a str, Symbol>,
    strings: TiVec<Symbol, &'a str>,
}

impl<'a> SymbolMap<'a> {
    pub fn new() -> Self {
        let mut names = HashMap::new();
        let mut strings = TiVec::new();

        for kw in KEYWORDS {
            let name = kw.as_str();
            let symbol = kw.as_symbol();

            names.insert(name, symbol);
            strings.push(name);
        }

        Self { names, strings }
    }

    pub fn insert(&mut self, string: &'a str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol::new(self.names.len());
        self.strings.push(string);
        self.names.insert(string, name);

        name
    }

    pub fn get<S: Into<Symbol>>(&self, symbol: S) -> &'a str {
        self.strings[symbol.into()]
    }

    pub const fn is_keyword(&self, symbol: Symbol) -> bool {
        symbol.0 < KEYWORDS.len()
    }
}

macro_rules! keywords {
    ($( $name:ident : $string:expr),* ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Kw {
            $(
                $name,
            )*
        }

        impl Kw {
            pub const fn as_str(&self) -> &'static str {
                match &self {
                    $(
                        Kw::$name => $string,
                    )*
                }
            }

            pub const fn as_symbol(&self) -> Symbol {
                Symbol::new(*self as usize)
            }
        }

        impl From<Kw> for Symbol {
            fn from(kw: Kw) -> Symbol {
                kw.as_symbol()
            }
        }

        pub const KEYWORDS: &'static [Kw] = &[
            $(
                Kw::$name,
            )*
        ];
    };
}

keywords![
    Let: "let",
    If: "if",
    Else: "else",
    True: "true",
    False: "false",
    Println:"println",

    Loop: "loop",
    Break: "break",
    Continue: "continue",

    I32: "i32",
    Bool: "bool"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword() {
        let mut m = SymbolMap::new();

        assert_eq!(Kw::Let.as_symbol(), m.insert("let"));
        assert_eq!(Kw::If.as_symbol(), m.insert("if"));
        assert_eq!(Kw::Else.as_symbol(), m.insert("else"));
        assert_eq!(Kw::True.as_symbol(), m.insert("true"));
        assert_eq!(Kw::False.as_symbol(), m.insert("false"));
        assert_eq!(Kw::Println.as_symbol(), m.insert("println"));

        assert_eq!(Kw::Loop.as_symbol(), m.insert("loop"));
        assert_eq!(Kw::Break.as_symbol(), m.insert("break"));
        assert_eq!(Kw::Continue.as_symbol(), m.insert("continue"));

        assert_eq!(Kw::I32.as_symbol(), m.insert("i32"));
        assert_eq!(Kw::Bool.as_symbol(), m.insert("bool"));
    }

    #[test]
    fn ident() {
        let mut m = SymbolMap::new();
        let foo = m.insert("foo");
        assert_eq!("foo", m.get(foo));

        let foo2 = m.insert("foo");
        assert_eq!(foo, foo2);
    }

    #[test]
    fn is_keyword() {
        let mut m = SymbolMap::new();
        let foo = m.insert("foo");

        assert!(m.is_keyword(Kw::Let.as_symbol()));
        assert!(m.is_keyword(Kw::Println.as_symbol()));
        assert!(!m.is_keyword(foo));
    }
}
