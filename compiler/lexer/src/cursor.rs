use std::str::Chars;

pub(crate) struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub(crate) fn reset_len_comsumed(&mut self) {
        self.initial_len = self.chars.as_str().len();
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
