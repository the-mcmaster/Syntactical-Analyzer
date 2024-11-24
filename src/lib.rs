use crate::io::{expected_read, open_file};
use crate::lexer::{StateMachine, Token};

/// Handler of all IO related functionality.
mod io;
/// Module for all lexical analysis types, implementations,
/// and the **lexical state machine**.
pub mod lexer;

/// Orangized storage of the unique error codes.
mod error_codes {
    /// There was a problem while parsing the passed-in arguments to the program.
    pub(crate) const CLI_PARSE_ERROR: i32 = 1;
    /// There was an IO problem opening the file.
    pub(crate) const OPEN_FILE_ERROR: i32 = 2;
    /// Encountered an error while reading the file.
    pub(crate) const BYTE_READ_ERROR: i32 = 3;

    /// There was a parse error in the program.
    pub(crate) const LEXICAL_ERROR: i32 = 4;
}

/// Opens the file, then builds the tokens/lexemes
/// from a state machine byte-by-byte
/// in 1 pass, in order.
/// 
/// Returns the constructed token-lexeme pairs in order.
pub fn get_lexemes() -> Vec<(Token, String)> {
    // Try to open the file
    let source = open_file();

    // Initialize the state machine for parsing
    let mut lexer_state_machine = StateMachine::new();

    // Continuously parses characters until EOF is reached
    let mut lexemes = source
        .map(|maybe_c| expected_read(maybe_c)) // Expect the next byte from the file, and report an io and exit otherwise.
        .filter_map(|byte: u8| lexer_state_machine.tick(byte)) // Tick the state machine by the input byte, keeping any flushed lexemes.
        .flatten() // Converts our iterator of batches into an iterator over all of the batches' items instead
        .collect::<Vec<_>>(); // Collect the iterator to a list

    // EOF has been reached. Finalize the state machine (send a dummy whitespace).
    if let Some(final_tokens) = lexer_state_machine.finalize() {
        lexemes.extend(final_tokens);
    }

    lexemes
}