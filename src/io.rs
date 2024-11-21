use std::env::args;

/// Parse the provided arguments for an input file path.
pub fn parse_args() -> String {
    // read program's argument, expecting only '-i' anywhere
    let found_argument = args()
        .enumerate()
        .find(|(_index, c)| c == "-i")
        .map(|(index, _c)| index);

    if found_argument.is_none() {
        eprintln!("ERROR - provide argument `-i` to input a file");
        std::process::exit(1)
    }
    
    let input_arg_index = found_argument.unwrap() + 1;

    // get the argument just after `-i` and expect it to be a file path
    let found_path = args().skip(input_arg_index).next();
    
    if found_path.is_none() {
        eprintln!("ERROR - please provide input file after argument `-i`");
        std::process::exit(1)
    }
    
    let input_file_path = found_path.unwrap();

    return input_file_path;
}

pub fn expected_read(maybe_c: Result<u8, std::io::Error>, input_path: &str) -> u8 {
    maybe_c.map_err(|_err| {
        println!("ERROR - cannot read contents of {input_path}");
        std::process::exit(3)
    })
    .unwrap()
}