//! # Terminal Tokens
//! 
//! This module is dedicated to every terminal token type.
//! 
//! The types defined in this module may seem familiar to the `Token` variant
//! from the lexical analyzer: this is no coincidence.
//! 
//! All terminal token types have only two parts:
//! - Lexical Token Type
//! - Associated String Lexeme
//! 
//! Currently, the token is stored, but completely unused. This is helpful for
//! any future use, especially semantic analysis.
//! 
//! ## Macros
//! 
//! The implementation of `Parse` and `ParseDisplay` are essentially *trivial*.
//! This library defines one macro `impl_terminal_parse`
//! to preform these trivial implementations, with minimal information.
//! 
//! This saves 570 lines of code.

use q1_lib::lexer::Token;
use q1_lib::lexer::Symbol as Sym;

use crate::make_indent;
use crate::Parse;
use crate::ParseDisplay;

/// An extremely helpful DRY macro for trivially implementing `Parse` and `ParseDisplay` for terminal types.
/// 
/// This macro takes in 4 arguments:
/// - `SELF`: The type that the Parse is being implemented for.
/// - `token_pat => token`: 2 arguments sperated by `=>`.
///     - `token_pat`: some expected `Token` enum, as used in a match expression
///     - `token`: some expected resulting `Token`, which can use identifiers from `token_pat`
/// - `token_label`: some string label for the type of token to be expected.
/// 
/// Using these four arguments, the same template of code can be used to trivially implement
/// any terminal `Parse` implementation: it either is or it isn't.
/// 
/// Often `token_pat => token` will look identical on both sides,
/// which is basically just returning the same token.
/// 
/// Also, this macro will automatically derive ParseDisplay using $SELF.
/// 
/// - See `Parse` trait for how this library works.
/// - See `ParseDisplay` for how this library displays.
macro_rules! impl_terminal_parse {
    ($SELF: ty, $token_pat:pat => $token:expr, $token_label:expr) => {
        impl ParseDisplay for $SELF {
            fn display(&self, depth: usize, label: Option<String>) {
                let indent = make_indent(depth);
                let label = label.unwrap_or(Self::parse_label());
                println!("{indent}{label}: {}", self.lexeme_signature());
            }

            fn lexeme_signature(&self) -> String {
                self.lexeme.clone()
            }
        }
        impl Parse for $SELF {
            fn parse(buffer: &mut crate::ParseBuffer) -> Result<Self, String> {
                // We must expect at least *something*,
                // so we throw an error if there isnt
                if buffer.peek().is_none() {
                    Err(format!("Expected `{}`, but found nothing instead", <$SELF>::parse_label()))?
                }
                
                let mut fork = buffer.fork();
                // With that, we consume the next token in the parse buffer, and match its token.
                Ok(match fork.next().unwrap() {
                    // If it is the correct token pattern (Ex. `Token::Symbol(syn)`), then return the struct
                    ($token_pat, lexeme) => {
                        *buffer = fork;
                        Self {
                            token: $token,
                            lexeme
                        }
                    },
                    // otherwise, throw an error
                    (_token, lexeme) => Err(format!("Expected `{}`, but found `{lexeme}` instead", <$SELF>::parse_label()))?
                })
            }

            fn parse_label() -> String {
                format!("{}", $token_label)
            }
        }
    };
}

#[derive(Clone, Copy)]
pub struct Identifier {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Identifier, Token::Identifier => Token::Identifier, "{identifier}");

#[derive(Clone, Copy)]
pub struct Type {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Type, Token::Type(type_token) => Token::Type(*type_token), "{type}");

#[derive(Clone, Copy)]
pub struct Equals {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Equals, Token::Symbol(Sym::Equal) => Token::Symbol(Sym::Equal), "=");

#[derive(Clone, Copy)]
pub struct Semicolon {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Semicolon, Token::Symbol(Sym::Semicolon) => Token::Symbol(Sym::Semicolon), ";");

#[derive(Clone, Copy)]
pub struct Return {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Return, Token::Return => Token::Return, "return");

#[derive(Clone, Copy)]
pub struct Literal {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Literal, Token::Literal(literal) => Token::Literal(*literal), "{literal}");

#[derive(Clone, Copy)]
pub struct LeftParen {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(LeftParen, Token::Symbol(Sym::LeftParen) => Token::Symbol(Sym::LeftParen), "(");

#[derive(Clone, Copy)]
pub struct RightParen {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(RightParen, Token::Symbol(Sym::RightParen) => Token::Symbol(Sym::RightParen), ")");

#[derive(Clone, Copy)]
pub struct Plus {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Plus, Token::Symbol(Sym::Plus) => Token::Symbol(Sym::Plus), "+");

#[derive(Clone, Copy)]
pub struct Minus {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Minus, Token::Symbol(Sym::Minus) => Token::Symbol(Sym::Minus), "-");

#[derive(Clone, Copy)]
pub struct Multiply {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Multiply, Token::Symbol(Sym::Multiply) => Token::Symbol(Sym::Multiply), "*");

#[derive(Clone, Copy)]
pub struct Divide {
    pub token: Token,
    pub lexeme: &'static String,
}
impl_terminal_parse!(Divide, Token::Symbol(Sym::Divide) => Token::Symbol(Sym::Divide), "/");

#[derive(Clone, Copy)]
pub struct Comma {
    pub token: Token,
    pub lexeme: &'static String
}
impl_terminal_parse!(Comma, Token::Symbol(Sym::Comma) => Token::Symbol(Sym::Comma), ",");

#[derive(Clone, Copy)]
pub struct LeftCurly {
    pub token: Token,
    pub lexeme: &'static String
}
impl_terminal_parse!(LeftCurly, Token::Symbol(Sym::LeftCurly) => Token::Symbol(Sym::LeftCurly), "{");

#[derive(Clone, Copy)]
pub struct RightCurly {
    pub token: Token,
    pub lexeme: &'static String
}
impl_terminal_parse!(RightCurly, Token::Symbol(Sym::RightCurly) => Token::Symbol(Sym::RightCurly), "}");