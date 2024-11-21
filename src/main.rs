mod lexer;
mod io;

use std::fs::File;
use std::io::Bytes;
use std::io::Read;

use crate::io::*;
use crate::lexer::*;

/// Returns `true` for any ascii whitespace characters.
fn is_whitespace(c: u8) -> bool {
    match c {
        0x9 | 0xA | 0xB | 0xC | 0xD | 0x20 => true,
        _ => false
    }
}

/// Compares a character literal and an 8-bit byte for equality.
fn matches(control: char, test: u8) -> bool {
    // Since `char` in Rust is UTF-8, `char` is 4 bytes.
    // Additionally, an unsigned 8-bit can be safely casted to an unsigned 32-bit.
    (control as u32) == (test as u32)
}

#[derive(Clone, Copy)]
enum State {
    /// Countinue to skip whitespace characters until a non-whitespace is selected.
    ScrollToNext,
    /// Expecting a number
    Number,
        /// NumberDigit expects a decimal literal.
        /// This can be promoted to NumberFloat when "." is encountered later on.
        NumberDigit, NumberFloat,
        
        // VVV Probably not neccessary for this assignment VVV
        /* /// NumberHex expects a hexadecimal literal.
        NumberHex, */

    /// Expecting a word (ident/type)
    Identifier,
    
    MaybeTypeInt2,
    MaybeTypeInt3,
    ConfirmTypeInt,
    
    MaybeTypeFloat2,
    MaybeTypeFloat3,
    MaybeTypeFloat4,
    MaybeTypeFloat5,
    ConfirmTypeFloat,

    MaybeKeywordReturn2,
    MaybeKeywordReturn3,
    MaybeKeywordReturn4,
    MaybeKeywordReturn5,
    MaybeKeywordReturn6,
    ConfirmKeywordReturn,
}

struct StateMachine {
    state: State,
    lexeme: String,
}
impl StateMachine {
    /// Creates a new state machine for lexical analysis.
    /// 
    /// The starting state is expecting 0 or more whitespace,
    /// with an empty lexeme buffer.
    fn new() -> Self {
        Self {
            state: State::ScrollToNext,
            lexeme: "".into(),
        }
    }

    /// Report an error with a given error message, and exit the program.
    fn detonate(&self, err_msg: String) -> ! {
        eprintln!("ERROR - failed to parse lexemes: {err_msg}");
        std::process::exit(1)
    }
    
    /// Advances the state machine by a singular byte,
    /// updating the internal state of the state machine.
    /// 
    /// If a lexeme has been detected as complete,
    /// this function will return `Some`.
    /// Otherwise, this will return `None`.
    /// 
    /// It is the user's responsibility to know when the input has ended, and
    /// then use `finalize`.
    fn tick(&mut self, c: u8) -> Option<Vec<(Token, String)>> {
        // DRY (Don't repeat yourself) macro, which expects a token type as input,
        // (which is used as the output's token type),
        // resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_lexeme_as_token {
            ($token:expr) => {{
                let output = ($token, self.lexeme.clone());

                self.state = State::ScrollToNext;
                self.lexeme = "".into();

                return Some(vec![output]);
            }}
        }

        // DRY (Don't repeat yourself) macro, which expects a character,
        // (which is used as the symbol token's lexeme),
        // resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_symbol_as_token {
            ($symbol:expr, $lexeme:expr) => {{
                let output = ($symbol.into(), {$lexeme}.into());

                self.state = State::ScrollToNext;
                self.lexeme = "".into();

                return Some(vec![output]);
            }};
        }

        macro_rules! flush_lexeme_and_symbol_as_token {
            ($lexeme_token:expr, ($symbol:expr, $symbol_lexeme:expr)) => {{
                let mut output = vec![($lexeme_token, self.lexeme.clone())];
                output.push(({$symbol}.into(), {$symbol_lexeme}.into()));

                self.state = State::ScrollToNext;
                self.lexeme = "".into();

                return Some(output);
            }};
        }
        use CharClass::*;
        use lexer::Symbol as Sym;
        use lexer::Type as Ty;
        match self.state {
            State::ScrollToNext if is_whitespace(c) => None,
            State::ScrollToNext => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('i', c) => State::MaybeTypeInt2,
                    Letter if matches('f', c) => State::MaybeTypeFloat2,
                    Letter if matches('r', c) => State::MaybeKeywordReturn2,
                    Letter | Symbol(Sym::Underscore) => State::Identifier,
                    Digit => State::Number,
                    Symbol(sym) => flush_symbol_as_token!(sym, c as char),
                    Unknown => self.detonate(format!("Unknown character `0x{c:x}`")),

                };

                self.lexeme.push(c as char);

                None
            },



            State::Number if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralInt),
            State::Number => {
                self.state = match CharClass::parse(c) {
                    Digit => State::NumberDigit,
                    Symbol(Sym::Period) => State::NumberFloat,
                    
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::LiteralInt, (sym, c as char)),
                    
                    _ => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);
                
                None
            },



            State::NumberDigit if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralInt),
            State::NumberDigit => {
                self.state = match CharClass::parse(c) {
                    Digit => State::NumberDigit,
                    Symbol(Sym::Period) => State::NumberFloat,
                    
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::LiteralInt, (sym, c as char)),
                    
                    _ => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);
                
                None
            },



            State::NumberFloat if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralFloat),
            State::NumberFloat => {
                self.state = match CharClass::parse(c) {
                    Digit => State::NumberDigit,
                    
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::LiteralFloat, (sym, c as char)),
                    
                    _ => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);
                
                None
            },
            

            State::Identifier if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::Identifier => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    
                    Symbol(sym) => {
                        let mut output = vec![(Token::LiteralInt, self.lexeme.clone())];
                        output.push((sym.into(), (c as char).into()));
        
                        self.state = State::ScrollToNext;
                        self.lexeme = "".into();
        
                        return Some(output);
                    },
                    
                    _ => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme))
                };

                self.lexeme.push(c as char);

                None
            },



            State::MaybeTypeInt2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('n', c) => State::MaybeTypeInt3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },


            State::MaybeTypeInt3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::ConfirmTypeInt,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },

            State::ConfirmTypeInt if is_whitespace(c) => flush_lexeme_as_token!(Ty::Int.into()),
            State::ConfirmTypeInt => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };
                
                self.lexeme.push(c as char);

                None
            }
            


            State::MaybeTypeFloat2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('l', c) => State::MaybeTypeFloat3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            


            State::MaybeTypeFloat3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('o', c) => State::MaybeTypeFloat4,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            
            
            State::MaybeTypeFloat4 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat4 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('a', c) => State::MaybeTypeFloat5,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            
            
            State::MaybeTypeFloat5 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat5 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::ConfirmTypeFloat,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },

            State::ConfirmTypeFloat if is_whitespace(c) => flush_lexeme_as_token!(Ty::Float.into()),
            State::ConfirmTypeFloat => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };
                
                self.lexeme.push(c as char);

                None
            },

            

            State::MaybeKeywordReturn2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeKeywordReturn2 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('e', c) => State::MaybeKeywordReturn3,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },

            State::MaybeKeywordReturn3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeKeywordReturn3 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('t', c) => State::MaybeKeywordReturn4,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            State::MaybeKeywordReturn4 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeKeywordReturn4 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('u', c) => State::MaybeKeywordReturn5,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            State::MaybeKeywordReturn5 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeKeywordReturn5 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('r', c) => State::MaybeKeywordReturn6,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            State::MaybeKeywordReturn6 if is_whitespace(c) => flush_lexeme_as_token!(Token::Return),
            State::MaybeKeywordReturn6 => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('n', c) => State::ConfirmKeywordReturn,
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,

                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };

                self.lexeme.push(c as char);

                None
            },
            
            State::ConfirmKeywordReturn if is_whitespace(c) => flush_lexeme_as_token!(Token::Return),
            State::ConfirmKeywordReturn => {
                self.state = match CharClass::parse(c) {
                    Letter | Symbol(Sym::Underscore) | Digit => State::Identifier,
                    Symbol(sym) => flush_lexeme_and_symbol_as_token!(Token::Identifier, (sym, c as char)),
                    Unknown => self.detonate(format!("Unexpected character `0x{c:x}` after `{}`", self.lexeme)),
                };
                
                self.lexeme.push(c as char);

                None
            },
        }
    }

    /// Completes the state machine, outputting a lexeme if one exists.
    /// 
    /// This is useful to use once EOF has been reached from the input source.
    /// 
    /// This function is identical to matching a whitespace.
    fn finalize(mut self) -> Option<Vec<(Token, String)>> {
        self.tick(0xA)
    }
}


fn get_lexemes(source: Bytes<File>, input_path: &str) -> Vec<(Token, String)> {
    // initialize the state machine for parsing
    let mut lexer_state_machine = StateMachine::new();

    // continuously parses characters until EOF is reached
    let mut lexemes = source
        .map(|maybe_c| expected_read(maybe_c, input_path)) // Expect the next byte from the file, and report an io and exit otherwise.
        .filter_map(|byte| lexer_state_machine.tick(byte)) // Tick the state machine by the input byte, keeping any flushed lexemes.
        .flatten() // Converts our iterator of batches into an iterator over all of the batches' items instead
        .collect::<Vec<_>>(); // Collect the iterator to a list

    // EOF has been reached
    if let Some(final_tokens) = lexer_state_machine.finalize() {
        lexemes.extend(final_tokens);
    }

    lexemes
}

fn main() {
    let input_path: &str = &parse_args();
    let source = File::open(input_path).unwrap().bytes();

    let lexemes = get_lexemes(source, input_path);

    println!("{:<32}|{}", "TOKEN", "LEXEME");
    for (token, lexeme) in lexemes {
        println!("{:<32} {}\n", format!("{token:?}"), lexeme)
    }
    
}