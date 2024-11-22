#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Token {
    LiteralInt,
    LiteralFloat,
    Identifier,
    Symbol(Symbol),
    Type(Type),
    Return,
}
impl From<Symbol> for Token {
    fn from(sym: Symbol) -> Self {
        Token::Symbol(sym)
    }
}
impl From<Type> for Token {
    fn from(ty: Type) -> Self {
        Token::Type(ty)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    // Arithmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Assignment Operator
    Equal,
    Semicolon,

    // Grouping Operators
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,

    // Underscore: for indentifiers
    Underscore,

    // Comma: for method arguments
    Comma,

    // Period: for floating point
    Period,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharClass {
    /// [a-zA-Z]
    Letter,

    /// [0-9]
    Digit,

    /// [+-*/=;(){}_,.]
    Symbol(Symbol),

    /// An unexpected character was parsed...
    Unknown,
}
impl CharClass {
    /// Parses a byte, expecting a 7-bit ascii code.
    pub fn parse(c: u8) -> Self {
        // Expect only certain range of characters from the non-extended ascii table
        if c < 0x21 || 0x7E < c {
            return Self::Unknown;
        }

        match c as char {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B'
            | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P'
            | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => Self::Letter,

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Digit,

            '+' => Symbol::Plus.into(),
            '-' => Symbol::Minus.into(),
            '*' => Symbol::Multiply.into(),
            '/' => Symbol::Divide.into(),

            '=' => Symbol::Equal.into(),
            ';' => Symbol::Semicolon.into(),

            '(' => Symbol::LeftParen.into(),
            ')' => Symbol::RightParen.into(),
            '{' => Symbol::LeftCurly.into(),
            '}' => Symbol::RightCurly.into(),

            '_' => Symbol::Underscore.into(),

            ',' => Symbol::Comma.into(),

            '.' => Symbol::Period.into(),

            _ => Self::Unknown,
        }
    }
}
impl From<Symbol> for CharClass {
    fn from(sym: Symbol) -> Self {
        CharClass::Symbol(sym)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Int,
    Float,
}
