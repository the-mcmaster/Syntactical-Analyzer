use std::process;

use q2_lib::{non_terminals::FunctionDefinition, Parse, ParseBuffer, ParseDisplay};

fn main() {
    let mut parse_buffer = ParseBuffer::new();
    match FunctionDefinition::parse(&mut parse_buffer) {
        Ok(function_definition) => {
            function_definition.display(0, None);
        },
        Err(err) => {
            eprintln!("PARSE ERROR:");
            eprintln!("{err}");
            process::exit(1);
        },
    }
}
