#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // One-character tokens
    Plus,
    Minus,
    Eq,
    Lt,
    Gt,

    Literal(Literal),

    // Multi-character tokens
    EqEq,
    Le,
    Ge,

    // Whitespace
    Whitespace,

    // Special tokens
    Eof,
    IllegalToken,
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    Integer,
}
