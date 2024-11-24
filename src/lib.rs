use crate::io::{expected_read, open_file};
use crate::lexer::{StateMachine, Token};

/// Handler of all IO related functionality.
mod io;
/// Module for all lexical analysis types, implementations,
/// and the **lexical state machine**.
pub mod lexer;

pub fn get_lexemes() -> Vec<(Token, String)> {
    // Try to open the file
    let source = open_file();

    // Initialize the state machine for parsing
    let mut lexer_state_machine = StateMachine::new();

    // Continuously parses characters until EOF is reached
    let mut lexemes = source
        .map(|maybe_c| expected_read(maybe_c)) // Expect the next byte from the file, and report an io and exit otherwise.
        .filter_map(|byte| lexer_state_machine.tick(byte)) // Tick the state machine by the input byte, keeping any flushed lexemes.
        .flatten() // Converts our iterator of batches into an iterator over all of the batches' items instead
        .collect::<Vec<_>>(); // Collect the iterator to a list

    // EOF has been reached. Finalize the state machine (send a dummy whitespace).
    if let Some(final_tokens) = lexer_state_machine.finalize() {
        lexemes.extend(final_tokens);
    }

    lexemes
}