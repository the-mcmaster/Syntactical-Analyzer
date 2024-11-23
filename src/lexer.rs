/// The cream-of-the-crop (it always rises to the top) of this
/// assignment: the Token enum. This token tags a lexeme for the
/// syntactical analysis.
#[derive(Clone, Copy, Debug)]
pub enum Token {
    LiteralInt,
    LiteralFloat,
    Identifier,
    #[allow(dead_code)]
    Symbol(Symbol),
    #[allow(dead_code)]
    Type(Type),
    Return,
}
impl From<Symbol> for Token {
    fn from(sym: Symbol) -> Self {
        Token::Symbol(sym)
    }
}
impl From<Type> for Token {
    fn from(ty: Type) -> Self {
        Token::Type(ty)
    }
}

/// All the singleton character parseable symbols.
///
/// This includes
/// - Arithmetic Operators
/// - Assignment Operators
/// - Grouping Operators
/// - Identifier Underscore
/// - Comma/Period
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    // Arithmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Assignment Operator
    Equal,
    Semicolon,

    // Grouping Operators
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,

    // Underscore: for indentifiers
    Underscore,

    // Comma: for method arguments
    Comma,

    // Period: for floating point
    Period,
}

/// A determinant of a grouping of a character.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CharClass {
    /// [a-zA-Z]
    Letter,

    /// [0-9]
    Digit,

    /// [+-*/=;(){}_,.]
    Symbol(Symbol),

    /// An unexpected character was parsed...
    Unknown,
}
impl CharClass {
    /// Parses a byte, expecting a 7-bit ascii code.
    pub fn parse(c: u8) -> Self {
        // Expect only certain range of characters from the non-extended ascii table
        if c < 0x21 || 0x7E < c {
            return Self::Unknown;
        }

        match c as char {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B'
            | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P'
            | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => Self::Letter,

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Digit,

            '+' => Symbol::Plus.into(),
            '-' => Symbol::Minus.into(),
            '*' => Symbol::Multiply.into(),
            '/' => Symbol::Divide.into(),

            '=' => Symbol::Equal.into(),
            ';' => Symbol::Semicolon.into(),

            '(' => Symbol::LeftParen.into(),
            ')' => Symbol::RightParen.into(),
            '{' => Symbol::LeftCurly.into(),
            '}' => Symbol::RightCurly.into(),

            '_' => Symbol::Underscore.into(),

            ',' => Symbol::Comma.into(),

            '.' => Symbol::Period.into(),

            _ => Self::Unknown,
        }
    }
}
impl From<Symbol> for CharClass {
    fn from(sym: Symbol) -> Self {
        CharClass::Symbol(sym)
    }
}

/// A type keyword.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Int,
    Float,
}

/// Returns `true` for any ascii whitespace characters.
fn is_whitespace(c: u8) -> bool {
    match c {
        0x9 | 0xA | 0xB | 0xC | 0xD | 0x20 => true,
        _ => false,
    }
}

/// Compares a character literal and an 8-bit byte for equality.
fn matches(control: char, test: u8) -> bool {
    // Since `char` in Rust is UTF-8, `char` is 4 bytes.
    // Additionally, an unsigned 8-bit can be safely casted to an unsigned 32-bit.
    (control as u32) == (test as u32)
}

/// Exhaustively, all possible states of the state machine.
///
/// ### Note
/// For special keywords like
/// - `int`
/// - `float`
/// - `return`
/// there are *n* unique states, with *n* being the number
/// of characters in a keyword.
///
/// This is due to the keywords using the same
/// characters as identifiers, hence the reason
/// each one of these special states fail-safe
/// the state back to an identifier.
#[derive(Clone, Copy)]
enum State {
    /// Countinue to skip whitespace characters until a non-whitespace is selected.
    /// When a non-whitespace character is encountered, depatch to the next state.
    ScrollToNext,
    /// Expecting a decimal literal.
    /// This can be promoted to NumberFloat if '.' is encountered later on.
    NumberDigit,
    /// Parsing the decimal part of the floating point number.
    NumberFloat,

    /// Expecting an identifier.
    /// This happens after other word possibilities (types/keywords) have been ruled out.
    Identifier,

    /// A word that is possibly the `int` keyword.
    /// Test the second letter for 'n'.
    /// If passed, go on to test the third letter, defaulting to identifier.
    MaybeTypeInt2,
    /// A word that is possibly the `int` keyword.
    /// Test the third letter for 't'.
    /// If passed, go on to test to confirm, defaulting to identifier.
    MaybeTypeInt3,
    /// Test that the lexeme is, in fact, the int keyword depending on the given byte.
    /// Only if it is a letter, underscore, or digit, it will not confirm.
    ConfirmTypeInt,

    /// A word that is possibly the `float` keyword.
    /// Test the second letter for 'l'.
    /// If passed, go on to test the third letter, defaulting to identifier.
    MaybeTypeFloat2,
    /// A word that is possibly the `float` keyword.
    /// Test the third letter for 'o'.
    /// If passed, go on to test the fourth letter, defaulting to identifier.
    MaybeTypeFloat3,
    /// A word that is possibly the `float` keyword.
    /// Test the fourth letter for 'a'.
    /// If passed, go on to test the fifth letter, defaulting to identifier.
    MaybeTypeFloat4,
    /// A word that is possibly the `float` keyword.
    /// Test the fifth letter for 't'.
    /// If passed, go on to test to confirm, defaulting to identifier.
    MaybeTypeFloat5,
    /// Test that the lexeme is, in fact, the float keyword depending on the given byte.
    /// Only if it is a letter, underscore, or digit, it will not confirm.
    ConfirmTypeFloat,

    /// A word that is possibly the `return` keyword.
    MaybeKeywordReturn2,
    /// A word that is possibly the `return` keyword.
    MaybeKeywordReturn3,
    /// A word that is possibly the `return` keyword.
    MaybeKeywordReturn4,
    /// A word that is possibly the `return` keyword.
    MaybeKeywordReturn5,
    /// A word that is possibly the `return` keyword.
    MaybeKeywordReturn6,
    /// A word that is possibly the `return` keyword.
    ConfirmKeywordReturn,
}

pub struct StateMachine {
    state: State,
    lexeme: String,
}
impl StateMachine {
    /* PRIVATE METHODS */

    /// Hard resets the state machine,
    /// erasing the lexeme and going into its default state
    fn reset(&mut self) {
        self.state = State::ScrollToNext;
        self.lexeme.truncate(0);
    }

    /// Report an error with a given error message, and exit the program.
    fn detonate(&self, err_msg: String) -> ! {
        eprintln!("ERROR - failed to parse lexemes: {err_msg}");
        std::process::exit(1)
    }

    /* PUBLIC METHODS */

    /// Creates a new state machine for lexical analysis.
    ///
    /// The starting state is expecting 0 or more whitespace,
    /// with an empty lexeme buffer.
    pub fn new() -> Self {
        Self {
            state: State::ScrollToNext,
            lexeme: "".into(),
        }
    }

    /// Completes the state machine, outputting a lexeme if one exists.
    ///
    /// This is useful to use once EOF has been reached from the input source.
    ///
    /// This function is identical to matching a whitespace.
    pub fn finalize(mut self) -> Option<Vec<(Token, String)>> {
        self.tick(0xA)
    }

    /// # Description
    ///
    /// Advances the state machine by a singular byte,
    /// updating the internal state of the state machine.
    ///
    /// If one or more lexemes have been detected as complete,
    /// this function will return `Some`.
    /// Otherwise, this will return `None`.
    ///
    /// It is the user's responsibility to know when the input has ended, and
    /// then use `finalize`.
    ///
    /// ## Special notes to the grader...
    ///
    /// This function defines 3 macros, meant to greatly reduce boilerplate code
    /// of a repeating design pattern, written only for the scope of `tick`. This
    /// is intended to make the code more readable and maintainable.
    ///
    /// This is important to mention, because this function
    /// returns `Some(_)` rather than `None` if and only if
    ///
    /// 1. It was called through 1 of the 3 macros, and
    /// 2. The state machine was reset.
    ///
    /// Hense, the verbage of "flush" in each of the macros.
    pub fn tick(&mut self, c: u8) -> Option<Vec<(Token, String)>> {
        //
        use crate::lexer::Symbol as Sym;
        use CharClass::*;
        use Type as Ty;

        /// DRY (Don't repeat yourself) macro, which expects a token type as input,
        /// (which is used as the output's token type),
        /// resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_lexeme_as_token {
            ($token:expr) => {{
                let output = ($token, self.lexeme.clone());

                self.reset();

                return Some(vec![output]);
            }};
        }

        /// DRY (Don't repeat yourself) macro, which expects a character,
        /// (which is used as the symbol token's lexeme),
        /// resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_symbol_as_token {
            ($symbol:expr, $lexeme:expr) => {{
                let output = ($symbol.into(), { $lexeme }.into());

                self.reset();

                return Some(vec![output]);
            }};
        }

        /// Essentially an ordered combination of `flush_lexeme_as_token` then `flush_symbol_as_token`,
        /// but with only 1 return.
        ///
        /// DRY (Don't repeat yourself) macro, which expects a token type as input,
        /// (which is used as the current lexeme's token type)
        /// and the information for the symbol token
        /// (the symbol type and the symbol lexeme),
        /// resets the state machine, and returns the tokenized lexemes.
        macro_rules! flush_lexeme_and_symbol_as_token {
            ($lexeme_token:expr, ($symbol:expr, $symbol_lexeme:expr)) => {{
                let mut output = vec![($lexeme_token, self.lexeme.clone())];
                output.push(({ $symbol }.into(), { $symbol_lexeme }.into()));

                self.reset();

                return Some(output);
            }};
        }

        match self.state {
            State::ScrollToNext if is_whitespace(c) => None,
            State::ScrollToNext => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('i', c) => State::MaybeTypeInt2,
                    Letter if matches('f', c) => State::MaybeTypeFloat2,
                    Letter if matches('r', c) => State::MaybeKeywordReturn2,
                    Letter | Symbol(Sym::Underscore) => State::Identifier,
                    Digit => State::NumberDigit,
                    Symbol(sym) => flush_symbol_as_token!(sym, c as char),
                    Unknown => self.detonate(format!("Unknown character `0x{c:x}`")),
                };

                self.lexeme.push(c as char);

                None
            }

            State::NumberDigit if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralInt),
            State::NumberDigit => {
                self.state = match CharClass::parse(c) {
                    Digit => State::NumberDigit,
                    Symbol(Sym::Period) => State::NumberFloat,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::LiteralInt, (sym, c as char))
                    }

                    _ => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::NumberFloat if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralFloat),
            State::NumberFloat => {
                self.state = match CharClass::parse(c) {
                    Digit => State::NumberDigit,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::LiteralFloat, (sym, c as char))
                    }

                    _ => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::Identifier if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::Identifier => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::LiteralInt, (sym, c as char));
                    }

                    _ => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeInt2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('n', c) => State::MaybeTypeInt3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeInt3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::ConfirmTypeInt,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::ConfirmTypeInt if is_whitespace(c) => flush_lexeme_as_token!(Ty::Int.into()),
            State::ConfirmTypeInt => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }
                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeFloat2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('l', c) => State::MaybeTypeFloat3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeFloat3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('o', c) => State::MaybeTypeFloat4,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeFloat4 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat4 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('a', c) => State::MaybeTypeFloat5,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeTypeFloat5 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat5 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::ConfirmTypeFloat,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::ConfirmTypeFloat if is_whitespace(c) => flush_lexeme_as_token!(Ty::Float.into()),
            State::ConfirmTypeFloat => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }
                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeKeywordReturn2 if is_whitespace(c) => {
                flush_lexeme_as_token!(Token::Identifier)
            }
            State::MaybeKeywordReturn2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('e', c) => State::MaybeKeywordReturn3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeKeywordReturn3 if is_whitespace(c) => {
                flush_lexeme_as_token!(Token::Identifier)
            }
            State::MaybeKeywordReturn3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::MaybeKeywordReturn4,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeKeywordReturn4 if is_whitespace(c) => {
                flush_lexeme_as_token!(Token::Identifier)
            }
            State::MaybeKeywordReturn4 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('u', c) => State::MaybeKeywordReturn5,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeKeywordReturn5 if is_whitespace(c) => {
                flush_lexeme_as_token!(Token::Identifier)
            }
            State::MaybeKeywordReturn5 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('r', c) => State::MaybeKeywordReturn6,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::MaybeKeywordReturn6 if is_whitespace(c) => flush_lexeme_as_token!(Token::Return),
            State::MaybeKeywordReturn6 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('n', c) => State::ConfirmKeywordReturn,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }

                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }

            State::ConfirmKeywordReturn if is_whitespace(c) => {
                flush_lexeme_as_token!(Token::Return)
            }
            State::ConfirmKeywordReturn => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => {
                        flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char))
                    }
                    Unknown => self.detonate(format!(
                        "Unexpected character `0x{c:x}` after `{}`",
                        self.lexeme
                    )),
                };

                self.lexeme.push(c as char);

                None
            }
        }
    }
}
