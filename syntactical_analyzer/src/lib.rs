//! # q2_lib
//! 
//! This library contains two traits (`Parse` and `ParseDisplay`), which nearly
//! all types have an implementation for.
//! 
//! `Parse` is the main recursion, which this library has all implementations
//! using left-recursion.
//! 
//! `ParseDisplay`, along with the helper function `make_indent`, leverages
//! recursion once again to automatically format and display a type's
//! tree structure.
//! 
//! The syntactical structures of this library is organized as
//! - `terminals`: All barebone token types from the lexical analysis (the primative structures).
//! - `non-terminals`: All composite syntax structure (build off of more primative structures).
//! - `modulars`: Automatic list-like syntax parsers.

use std::{
    slice::Iter, // The iterator-type over slice structures
    iter::Peekable, // When used on `Iter`, it allows to "peekahead", without consumption
    sync::LazyLock // Used to safely use the `'static` lifetime, without having data as precondition.
};

use q1_lib::lexer::Token; // Reusing the token type defined in the first problem.

/// All parseable terminal tokens
pub mod terminals;
/// All parseable non-terminal tokens
pub mod non_terminals;
/// All list-pattern abstractions.
pub mod modulars;

/// The input token stream. This relies on the lexical analyzer from `Q1`.
/// 
/// The LazyLock guarentees the existance of `Vec<_>` at the static variable's
/// first use, and then keeping it immutable for the program's lifetime.
/// This allows the implementation to depend on the `'static` lifetime.
/// 
/// For more details on how the `Vec<_>` is obtained, see `q1_lib` in `Q1`.
static TOKEN_STREAM: LazyLock<Vec<(Token, String)>> = LazyLock::new(|| q1_lib::get_lexemes());

/// A helper function to make consistent indentation for a specified depth.
pub fn make_indent(depth: usize) -> String {
    let mut indent = String::new();
    let indent_piece = "    ".chars();
    for _ in 0..depth {
        indent.extend(indent_piece.clone());
    }
    indent
}

/// The skeleton of this library.
pub trait Parse<T = Self>
where Self: Sized + ParseDisplay {
    /// The main tool for parsing the token stream.
    /// 
    /// # Return Assumptions
    /// If a parse was successful (returned `Ok(_)`), then this function advances
    /// the parse buffer to "consume" the parsed item.
    /// 
    /// If a parse failed (returned `Err(_)`),
    /// then this function returns the buffer as it was,
    /// staying at the same position as passed in.
    /// 
    /// Deviance off of these assumptions are considered as a bug.
    /// 
    /// # Implementation Design Patterns
    /// 
    /// Nearly all implementations follow a similar forking pattern.
    /// 
    /// Here's a dummy example
    /// ```
    /// impl Parse for YourType {
    ///     fn parse(buffer: &mut ParseBuffer) -> Result<YourType, String> {
    ///         let mut fork = buffer.fork();
    ///
    ///         todo!("... attempt to parse on the fork ...");
    ///         
    ///         // parse was successful, so modify the buffer before returning
    ///         *buffer = fork;
    ///         return Ok(YourType);
    ///         
    ///         // or
    ///         
    ///         // parse was unsuccessful, so let the fork die in this scope
    ///         Err(format!("PARSE ERROR MESSAGE"))?
    ///     }
    ///     
    ///     fn parse_label() -> String {
    ///         todo!()
    ///     }
    /// }
    /// ```
    fn parse(buffer: &mut ParseBuffer) -> Result<T, String>;

    /// The label to be used to describe itself as a parse error
    fn parse_label() -> String;
}

/// An important tool for a parse tree to recursively display itself with correct
/// indenting.
pub trait ParseDisplay {
    /// The tool to print to stdout.
    /// 
    /// `depth` describes how deep the indentation should be. It is recommended
    /// to use `make_indent` to get correct indentation.
    /// 
    /// It is up to the implementor if the label will be used, or not, or at all.
    fn display(&self, depth: usize, label: Option<String>);

    /// The signature of all terminal lexemes, in-order, in a singular string.
    /// 
    /// This can be very long, especially for modular types like multi-statement blocks.
    /// If it is too verbose to include in `display`, still implement but disregard in
    /// the display.
    fn lexeme_signature(&self) -> String;
}

/// A cheaply-forkable iterator over a given token stream.
pub struct ParseBuffer {
    /// A peekable iterator over some known list of tokens and strings.
    buffer: Peekable<Iter<'static, (Token, String)>>
}
impl ParseBuffer {
    /// Create a new `ParseBuffer` over a token stream.
    /// 
    /// This will be the static token stream from the input file `TOKEN_STREAM`.
    /// 
    /// See `TOKEN_STREAM` for more details.
    pub fn new() -> Self {
        ParseBuffer { buffer: TOKEN_STREAM.iter().peekable() }
    }

    /// See if there is a "next" item, without actually consuming.
    pub fn peek(&mut self) -> Option<&(Token, String)> {
        self.buffer.peek().map(|&var| var)
    }

    /// Cheaply clone the buffer iterator at the buffer's current state.
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