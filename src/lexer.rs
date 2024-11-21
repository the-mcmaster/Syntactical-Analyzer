#[derive(Debug)]
pub enum Token {
    LiteralDecimal,
    LiteralHex,
    LiteralFloat,
    Identifier,
    Symbol
}