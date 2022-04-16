use std::borrow::Cow;
use std::iter;

const INDENT_SIZE: usize = 4;

pub enum Delim {
    Paren,
    Bracket,
    Brace,
}

impl Delim {
    pub fn as_open(&self) -> &'static str {
        match *self {
            Delim::Paren => "(",
            Delim::Bracket => "[",
            Delim::Brace => "{",
        }
    }

    pub fn as_close(&self) -> &'static str {
        match *self {
            Delim::Paren => ")",
            Delim::Bracket => "]",
            Delim::Brace => "}",
        }
    }
}

pub struct Printer {
    out: String,
    indent: usize,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            out: String::new(),
            indent: 0,
        }
    }

    pub fn finish(self) -> String {
        self.out
    }

    /// Begin indent.
    /// # Examples
    /// ```
    /// use printer::{Delim, Printer};
    /// 
    /// let mut p = Printer::new();
    /// 
    /// p.word("a ");
    /// p.begin(Delim::Brace);
    /// p.word("b");
    /// p.end(Delim::Brace);
    /// 
    /// let s = 
    /// r"a {
    ///     b
    /// }";
    /// 
    /// 
    /// assert_eq!(s, p.finish());
    /// ```
    pub fn begin(&mut self, delim: Delim) {
        self.word(delim.as_open());
        self.indent += 1;
        self.new_line();
    }

    /// End indent.
    /// # Examples
    /// ```
    /// use printer::{Delim, Printer};
    /// 
    /// let mut p = Printer::new();
    /// 
    /// p.word("a ");
    /// p.begin(Delim::Brace);
    /// p.word("b");
    /// p.end(Delim::Brace);
    /// 
    /// let s = 
    /// r"a {
    ///     b
    /// }";
    /// 
    /// 
    /// assert_eq!(s, p.finish());
    /// ```
    pub fn end(&mut self, delim: Delim) {
        self.indent -= 1;
        self.new_line();
        self.word(delim.as_close());
    }

    /// New line with indent.
    pub fn new_line(&mut self) {
        self.out.push('\n');
        self.out
            .extend(iter::repeat(' ').take(INDENT_SIZE * self.indent));
    }

    /// Print the given open [`Delim`]
    /// # Examples
    /// ```
    /// use printer::{Delim, Printer};
    /// 
    /// let mut p = Printer::new();
    /// 
    /// p.popen(Delim::Paren);
    /// p.popen(Delim::Bracket);
    /// p.popen(Delim::Brace);
    /// 
    /// assert_eq!("([{", p.finish());
    /// ```
    pub fn popen(&mut self, delim: Delim) {
        self.word(delim.as_open());
    }

    /// Print the given close [`Delim`]
    /// # Examples
    /// ```
    /// use printer::{Delim, Printer};
    /// 
    /// let mut p = Printer::new();
    /// 
    /// p.pclose(Delim::Paren);
    /// p.pclose(Delim::Bracket);
    /// p.pclose(Delim::Brace);
    /// 
    /// assert_eq!(")]}", p.finish());
    /// ```
    pub fn pclose(&mut self, delim: Delim) {
        self.word(delim.as_close());
    }

    /// Print the given word.
    /// # Examples
    /// ```
    /// use printer::{Delim, Printer};
    /// 
    /// let mut p = Printer::new();
    /// 
    /// p.word("foo");
    /// p.word(123.to_string());
    /// 
    /// assert_eq!("foo123", p.finish());
    /// ```
    pub fn word<S: Into<Cow<'static, str>>>(&mut self, w: S) {
        let s = w.into();
        self.out.push_str(&s);
    }
}