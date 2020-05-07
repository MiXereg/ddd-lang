pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // One-character tokens
    Plus,  // '+'
    Minus, // '-'
    Eq,    // '='
    Lt,    // '<'
    Gt,    // '>'

    Literal(Literal),

    // Multi-character tokens
    EqEq, // "=="
    Le,   // "<="
    Ge,   // ">="

    // Whitespace: '\t', '\n', '\r', ' ', U+000B, U+000C, U+0085, U+200E, U+200F, U+2028, U+2029
    Whitespace(u64),

    // Special tokens
    Eof,
    IllegalToken,
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    Integer,
    String(bool),
}
