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

/// Tests the input if it is [0-9] in ascii
fn is_decimal(c: u8) -> bool {
    match c {
        0x30..0x39 => true,
        _ => false,
    }
}

/// Tests the input if it is [0-9a-fA-F] in ascii
fn is_hexadecimal(c: u8) -> bool {
    let is_a_through_f = |c: u8| -> bool {
        match c {
            // [A-F]
            0x41..=0x46 |
            // [a-f]
            0x61..=0x66 => true,
            _ => false
        }
    };

    is_decimal(c) || is_a_through_f(c)
}

/// Compares a character literal and an 8-bit byte for equality.
fn matches(control: char, test: u8) -> bool {
    // Since `char` in Rust is UTF-8, `char` is 4 bytes.
    // Additionally, an unsigned 8-bit can be safely casted to an unsigned 32-bit.
    (control as u32) == (test as u32)
}

enum CharClass {
    /// [a-zA-Z]
    Letter,
    
    /// [0-9]
    Digit,
    
    /// _
    Underscore,

    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftCurly,
    /// }
    RightCurly,
    
    /// ;
    Semicolon,
    /// ,
    Comma,

    /// +
    Plus,
    /// -
    Minus,
    /// /
    ForwardSlash,
    /// *
    Asterisk,

    /// =
    Equal,

    
    /// An unexpected character was parsed...
    Unknown
}
impl CharClass {
    /// Parses a byte, expecting a 7-bit ascii code.
    fn parse(c: u8) -> CharClass {
        use CharClass::*;
        
        // No character is expected from the extended ascii table...
        if c >= 0x80 {
            return Unknown;
        }


        match c as char {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |
            'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |
            'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
            'v' | 'w' | 'x' | 'y' | 'z' |
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' |
            'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' |
            'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' |
            'V' | 'W' | 'X' | 'Y' | 'Z' => Letter,

            '_' => Underscore,
            
            '0' | '1' | '2' | '3' | '4' |
            '5' | '6' | '7' | '8' | '9' => Digit,

            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftCurly,
            '}' => RightCurly,
            ';' => Semicolon,
            ',' => Comma,
            '+' => Plus,
            '-' => Minus,
            '/' => ForwardSlash,
            '*' => Asterisk,
            '=' => Equal,

            _ => Unknown
        }
    }
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
    MaybeTypeFloat2,
    MaybeTypeFloat3,
    MaybeTypeFloat4,
    MaybeTypeFloat5,
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
    fn tick(&mut self, c: u8) -> Option<(Token, String)> {
        // DRY (Don't repeat yourself) macro, which expects a token type as input,
        // (which is used as the output's token type),
        // resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_lexeme_as_token {
            ($token:expr) => {{
                let output = ($token, self.lexeme.clone());

                self.state = State::ScrollToNext;
                self.lexeme = "".into();

                return Some(output);
            }}
        }

        // DRY (Don't repeat yourself) macro, which expects a character,
        // (which is used as the symbol token's lexeme),
        // resets the state machine, and returns the tokenized lexeme.
        macro_rules! flush_symbol_as_token {
            ($symbol:expr) => {{
                let output = (Token::Symbol, $symbol.into());

                self.state = State::ScrollToNext;
                self.lexeme = "".into();

                return Some(output);
            }};
        }
        use CharClass::*;
        match self.state {
            State::ScrollToNext if is_whitespace(c) => None,
            State::ScrollToNext => {
                self.state = match CharClass::parse(c) {
                    Letter if matches('i', c) => State::MaybeTypeInt2,
                    Letter if matches('f', c) => State::MaybeTypeFloat2,
                    Letter | Underscore => State::Identifier,
                    Digit => State::Number,
                    LeftParen => flush_symbol_as_token!(c as char),
                    RightParen => flush_symbol_as_token!(c as char),
                    LeftCurly => flush_symbol_as_token!(c as char),
                    RightCurly => flush_symbol_as_token!(c as char),
                    Semicolon => flush_symbol_as_token!(c as char),
                    Comma => flush_symbol_as_token!(c as char),
                    Plus => flush_symbol_as_token!(c as char),
                    Minus => flush_symbol_as_token!(c as char),
                    ForwardSlash => flush_symbol_as_token!(c as char),
                    Asterisk => flush_symbol_as_token!(c as char),
                    Equal => flush_symbol_as_token!(c as char),
                    Unknown => self.detonate(format!("Unknown character `{c:x}`")),

                };

                self.lexeme.push(c as char);

                None
            },



            State::Number if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralDecimal),
            State::Number => {
                self.state = if is_decimal(c) {
                    State::NumberDigit
                } else if matches('.', c) {
                    State::NumberFloat
                } else {
                    self.detonate(format!("Unexpected character `{c:x}` after `{}`", self.lexeme))
                };

                self.lexeme.push(c as char);
                
                None
            },



            State::NumberDigit if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralDecimal),
            State::NumberDigit => {
                self.state = if matches('.', c) {
                    State::NumberFloat
                } else if is_decimal(c) {
                    State::NumberDigit
                } else {
                    self.detonate(format!("Unexpected character `{c:x}` after `{}`", self.lexeme));
                };

                self.lexeme.push(c as char);
                
                None
            },



            State::NumberFloat if is_whitespace(c) => flush_lexeme_as_token!(Token::LiteralFloat),
            State::NumberFloat => {
                self.state = if is_decimal(c) {
                    State::NumberFloat // check if 1. is a valid float in c
                } else {
                    self.detonate(format!("Unexpected character `{c:x}` after `{}`", self.lexeme));
                };

                self.lexeme.push(c as char);
                
                None
            },
            

            State::Identifier if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::Identifier => {
                self.state = match CharClass::parse(c) {
                    Letter | Underscore | Digit => State::Identifier,
                    _ => self.detonate(format!("Unexpected character `{c:x}` after `{}`", self.lexeme))
                };

                self.lexeme.push(c as char);

                None
            },



            State::MaybeTypeInt2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt2 => todo!(),


            State::MaybeTypeInt3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeInt3 => todo!(),
            


            State::MaybeTypeFloat2 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat2 => todo!(),
            


            State::MaybeTypeFloat3 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat3 => todo!(),
            
            
            
            State::MaybeTypeFloat4 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat4 => todo!(),
            
            
            
            State::MaybeTypeFloat5 if is_whitespace(c) => flush_lexeme_as_token!(Token::Identifier),
            State::MaybeTypeFloat5 => todo!(),
        }
    }

    /// Completes the state machine, outputting a lexeme if one exists.
    /// 
    /// This is useful to use once EOF has been reached from the input source.
    /// 
    /// This function is identical to matching a whitespace.
    fn finalize(mut self) -> Option<(Token, String)> {
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
        .collect::<Vec<(Token, String)>>();

    // EOF has been reached
    if let Some(final_token) = lexer_state_machine.finalize() {
        lexemes.push(final_token);
    }

    lexemes
}

fn main() {
    let input_path: &str = &parse_args();
    let source = File::open(input_path).unwrap().bytes();

    
    
    let lexemes = get_lexemes(source, input_path);

    println!("{lexemes:?}")
}