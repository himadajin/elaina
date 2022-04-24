use std::{
    fmt::{Display, Write},
    ops::{AddAssign, SubAssign},
};

pub enum Delim {
    Paren,
    Bracket,
    Brace,
}

impl Delim {
    pub const fn as_open(&self) -> &'static str {
        match *self {
            Delim::Paren => "(",
            Delim::Bracket => "[",
            Delim::Brace => "{",
        }
    }

    pub const fn as_close(&self) -> &'static str {
        match *self {
            Delim::Paren => ")",
            Delim::Bracket => "]",
            Delim::Brace => "}",
        }
    }
}

pub trait Printer {
    const INDENT_SIZE: usize = 4;
    type Output: Write + Display;

    fn finish(self) -> Self::Output;
    fn get_output_mut(&mut self) -> &mut Self::Output;
    fn get_indent_mut(&mut self) -> &mut usize;

    /// Print given value to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.print("123");
    /// p.print("abc");
    /// assert_eq!("123abc", p.finish());
    /// ```
    fn print<T: Display>(&mut self, s: T) {
        write!(self.get_output_mut(), "{}", s).unwrap();
    }

    /// Print space and given value to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.space_print("abc");
    /// assert_eq!(" abc", p.finish());
    /// ```
    fn space_print<T: Display>(&mut self, s: T) {
        self.space();
        self.print(s);
    }

    /// Print space, given value and space to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.space_print_space("abc");
    /// assert_eq!(" abc ", p.finish());
    /// ```
    fn space_print_space<T: Display>(&mut self, s: T) {
        self.space();
        self.print(s);
        self.space();
    }

    /// Print given value and space to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.print_space("abc");
    /// assert_eq!("abc ", p.finish());
    /// ```
    fn print_space<T: Display>(&mut self, s: T) {
        self.print(s);
        self.space();
    }

    /// Print space to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.space();
    /// assert_eq!(" ", p.finish());
    /// ```
    fn space(&mut self) {
        self.print(' ');
    }

    /// Print multiple spaces to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.spaces(3);
    /// assert_eq!("   ", p.finish());
    /// ```
    fn spaces(&mut self, n: usize) {
        let mut s = String::with_capacity(n);
        s.extend(std::iter::repeat(' ').take(n));
        self.print(s);
    }

    /// Print `;` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.semi();
    /// assert_eq!(";", p.finish());
    /// ```
    fn semi(&mut self) {
        self.print(';');
    }

    /// Print `:` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.colon();
    /// assert_eq!(":", p.finish());
    /// ```
    fn colon(&mut self) {
        self.print(":");
    }

    /// Print `.` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.dot();
    /// assert_eq!(".", p.finish());
    /// ```
    fn dot(&mut self) {
        self.print(".");
    }

    /// Print `,` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.comma();
    /// assert_eq!(",", p.finish());
    /// ```
    fn comma(&mut self) {
        self.print(",");
    }

    /// Print `=` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.eq();
    /// assert_eq!("=", p.finish());
    /// ```
    fn eq(&mut self) {
        self.print("=");
    }

    /// Print `+` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.plus();
    /// assert_eq!("+", p.finish());
    /// ```
    fn plus(&mut self) {
        self.print("+");
    }

    /// Print `-` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.minus();
    /// assert_eq!("-", p.finish());
    /// ```
    fn minus(&mut self) {
        self.print("-");
    }

    /// Print `*` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.star();
    /// assert_eq!("*", p.finish());
    /// ```
    fn star(&mut self) {
        self.print("*");
    }

    /// Print `/` to output.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.slash();
    /// assert_eq!("/", p.finish());
    /// ```
    fn slash(&mut self) {
        self.print("/");
    }

    /// Print multiple items separated by `, ` and parensized.
    /// # Example
    /// ```
    /// use printer::*;
    /// 
    /// let v = vec![1, 2, 3, 4];
    /// let mut p = PrinterAnd::new(());
    /// p.list(v.iter(), Delim::Paren, |this, item| {
    ///     this.print(item);
    /// });
    /// assert_eq!("(1, 2, 3, 4)", p.finish());
    /// ```
    fn list<T, I, F>(&mut self, items: T, delim: Delim, f: F)
    where
        T: Iterator<Item = I>,
        F: Fn(&mut Self, I),
    {
        self.with_delim(delim, false, |this| {
            this.separated(
                items,
                |this| {
                    this.comma();
                    this.space();
                },
                f,
            );
        });
    }

    /// Add newline.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let mut p = PrinterAnd::new(());
    /// p.print(1234);
    /// p.newline();
    /// p.print(5678);
    /// p.newline();
    ///
    /// let out =
    /// r"1234
    /// 5678
    /// ";
    /// assert_eq!(out, p.finish());
    /// ```
    fn newline(&mut self) {
        self.print("\n");
        let margin = *self.get_indent_mut() * Self::INDENT_SIZE;
        self.print(String::from_iter(std::iter::repeat(' ').take(margin)));
    }

    /// Print multiple items on a newline.
    /// No line breaks at the end.
    /// # Example
    /// ```
    /// use printer::*;
    ///
    /// let v = vec![1, 2, 3, 4];
    /// let mut p = PrinterAnd::new(());
    /// p.lines(v.iter(), |this, line| {
    ///     this.print(line);
    ///     this.semi();
    /// });
    ///
    /// let out =
    ///r"1;
    ///2;
    ///3;
    ///4;";
    /// assert_eq!(out, p.finish());
    /// ```
    fn lines<I, T, F>(&mut self, lines: T, f: F)
    where
        T: Iterator<Item = I>,
        F: Fn(&mut Self, I),
    {
        self.separated(lines, |this| this.newline(), f);
    }

    /// Print multiple items with separator.
    /// No separator inserted at the end.
    /// # Examples
    /// ```
    /// use printer::*;
    ///
    /// let v = vec![1, 2, 3, 4];
    /// let mut p = PrinterAnd::new(());
    /// p.separated(
    ///     v.iter(),
    ///     |this| {
    ///         this.comma();
    ///         this.space()
    ///     },
    ///     |this, item| this.print(item),
    /// );
    ///
    /// assert_eq!("1, 2, 3, 4", p.finish());
    /// ```
    fn separated<I, T, S, F>(&mut self, items: T, sep: S, f: F)
    where
        T: Iterator<Item = I>,
        S: Fn(&mut Self),
        F: Fn(&mut Self, I),
    {
        let mut items = items.peekable();
        while let Some(item) = items.next() {
            f(self, item);

            if items.peek().is_some() {
                sep(self);
            }
        }
    }

    /// Print item with delimiter.
    /// # Examples
    /// ```
    /// use printer::*;
    ///
    /// {
    ///     let mut p = PrinterAnd::new(());
    ///     p.with_delim(Delim::Paren, false, |this| {
    ///         this.print(123);
    ///         this.print("abc");
    ///     });
    ///     assert_eq!("(123abc)", p.finish());
    /// }
    ///
    /// {
    ///     let mut p = PrinterAnd::new(());
    ///     p.with_delim(Delim::Brace, true, |this| {
    ///         let v = vec![1, 2, 3, 4];
    ///         this.lines(v.iter(), |this, line| {
    ///             this.print(line);
    ///             this.semi();
    ///         });
    ///     });
    ///     let out =
    ///r"{
    ///     1;
    ///     2;
    ///     3;
    ///     4;
    ///}";
    ///     assert_eq!(out, p.finish());
    /// }
    /// ```
    fn with_delim<F: FnOnce(&mut Self)>(&mut self, delim: Delim, indent: bool, f: F) {
        self.print(delim.as_open());
        if indent {
            self.get_indent_mut().add_assign(1);
            self.newline();
        }
        f(self);
        if indent {
            self.get_indent_mut().sub_assign(1);
            self.newline();
        }
        self.print(delim.as_close());
    }
}

pub struct PrinterAnd<T> {
    output: String,
    indent: usize,
    data: T,
}

impl<T> Printer for PrinterAnd<T> {
    const INDENT_SIZE: usize = 4;

    type Output = String;

    fn finish(self) -> Self::Output {
        self.output
    }

    fn get_output_mut(&mut self) -> &mut Self::Output {
        &mut self.output
    }

    fn get_indent_mut(&mut self) -> &mut usize {
        &mut self.indent
    }
}

impl<T> PrinterAnd<T> {
    pub fn new(data: T) -> PrinterAnd<T> {
        PrinterAnd {
            output: String::new(),
            indent: 0,
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
