use q1_lib::get_lexemes;

/// The main function.
///
/// Look in crate `q1_lib` for the backend implementation.
fn main() {
    // Get the tagged tokens, immutably storing it in lexemes.
    let lexemes = get_lexemes();

    println!("{:<24}|{}\n{:_<24}|{:_<24}", "TOKEN", "LEXEME", "", "");
    for (token, lexeme) in lexemes {
        println!("{:<24}|{}", format!("{token:?}"), lexeme)
    }
}
