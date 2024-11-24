use q1_lib::get_lexemes;

fn main() {
    // Get the tagged tokens, immutably storing it in lexemes.
    let lexemes = get_lexemes();

    println!("{:<24}|{}\n{:<24}|", "TOKEN", "LEXEME", "");
    for (token, lexeme) in lexemes {
        println!("{:<24}|{}", format!("{token:?}"), lexeme)
    }
}
