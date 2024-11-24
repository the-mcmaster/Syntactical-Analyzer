use std::{
    env::args,
    fs::File,
    io::{Bytes, Read},
    sync::LazyLock,
};

use crate::error_codes::{BYTE_READ_ERROR, CLI_PARSE_ERROR, OPEN_FILE_ERROR};

/// The input path passed-in from the CLI arguments, which is always expected.
///
/// This is purposely left private to compartmentalize the IO module.
/// 
/// LazyLock ensures that the value is loaded in static run-time memory
/// when first accessed, and ensures that the value is never mutated.
static INPUT_PATH: LazyLock<String> = LazyLock::new(|| {
    // read program's argument, expecting only '-i' anywhere
    let found_argument = args()
        .enumerate()
        .find(|(_index, c)| c == "-i")
        .map(|(index, _c)| index);

    // exit if the flag is not found.
    if found_argument.is_none() {
        eprintln!("ERROR - provide argument `-i` to input a file");
        std::process::exit(CLI_PARSE_ERROR)
    }

    // the next argument is the input path
    let input_arg_index = found_argument.unwrap() + 1;

    // get the argument just after `-i` and expect it to be a file path
    let found_path = args().skip(input_arg_index).next();

    if found_path.is_none() {
        eprintln!("ERROR - please provide input file after argument `-i`");
        std::process::exit(CLI_PARSE_ERROR)
    }

    let input_file_path = found_path.unwrap();

    return input_file_path;
});

/// Returns an interator over the bytes of a file.
///
/// The program will exit with an error message if the file cannot be opened.
pub fn open_file() -> Bytes<File> {
    match File::open(INPUT_PATH.as_str()) {
        Ok(file) => file.bytes(),

        Err(err) => {
            eprintln!(
                "ERROR - could not open file `{}` due to IO error - `{}`",
                INPUT_PATH.as_str(),
                err
            );
            std::process::exit(OPEN_FILE_ERROR)
        }
    }
}

/// Helper unwrapping function for an IO read of a byte.
///
/// If the input is `None`, the program exits with an error message.
/// Otherwise, the input is `Some(_)` and we safely unwrap the Result.
pub fn expected_read(maybe_c: Result<u8, std::io::Error>) -> u8 {
    maybe_c
        .map_err(|err| {
            println!(
                "ERROR - while reading byte at `{}` due to IO error - `{}`",
                INPUT_PATH.as_str(),
                err
            );
            std::process::exit(BYTE_READ_ERROR)
        })
        .unwrap()
}
