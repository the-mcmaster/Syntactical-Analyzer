use std::process;

use q2_lib::{
    Parse,
    ParseBuffer,
    ParseDisplay,
    non_terminals::FunctionDefinition
};

fn main() {
    // Get an original parse buffer at the start of the token stream.
    let mut parse_buffer = ParseBuffer::new();

    // Expect a function definition as the root structure. Try to parse it.
    match FunctionDefinition::parse(&mut parse_buffer) {
        // PARSE SUCCESS! Print it out!
        Ok(function_definition) => {
            function_definition.display(0, None);
        },

        // Something is wrong...
        //
        // Hopefully the input text!
        Err(err) => {
            eprintln!("PARSE ERROR:");
            eprintln!("{err}");
            process::exit(1);
        },
    }
}
