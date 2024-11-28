#![allow(warnings)]

use std::{iter::Peekable, slice::Iter, sync::LazyLock};

use q1_lib::lexer::Token;

mod terminals;
pub mod non_terminals;
mod modulars;

static TOKEN_STREAM: LazyLock<Vec<(Token, String)>> = LazyLock::new(|| q1_lib::get_lexemes());

pub fn make_indent(depth: usize) -> String {
    let mut indent = String::new();
    let indent_piece = "    ".chars();
    for _ in 0..depth {
        indent.extend(indent_piece.clone());
    }
    indent
}

pub trait Parse<T = Self>
where Self: Sized {
    fn parse(buffer: &mut ParseBuffer) -> Result<T, String>;

    fn parse_label() -> String;
}

pub trait ParseDisplay {
    fn display(&self, depth: usize, label: Option<String>);

    fn lexeme_signature(&self) -> String;
}

pub struct ParseBuffer {
    /// A peekable iterator over some known list of tokens and strings.
    buffer: Peekable<Iter<'static, (Token, String)>>
}
impl ParseBuffer {
    pub fn new() -> Self {
        ParseBuffer { buffer: TOKEN_STREAM.iter().peekable() }
    }

    pub fn peek(&mut self) -> Option<&(Token, String)> {
        self.buffer.peek().map(|&var| var)
    }

    pub fn fork(&self) -> Self {
        ParseBuffer { buffer: self.buffer.clone() }
    }
}
impl Iterator for ParseBuffer {
    type Item = &'static (Token, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.next()
    }
}