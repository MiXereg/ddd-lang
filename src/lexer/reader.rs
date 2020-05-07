use crate::token::{Literal, Token, TokenType};

pub struct Reader {
    pub src: String,
    pub tokens: Vec<Token>,
}

impl Reader {
    pub fn new(src: String) -> Reader {
        Reader {
            src: src,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let mut last_token_type: TokenType = TokenType::IllegalToken;

        // Iterator over src
        for c in self.src.chars() {
            let char_type: TokenType = check_char_type(c);

            let mut token_type: TokenType = check_token_type(char_type, last_token_type);

            match token_type {
                TokenType::IllegalToken => match char_type {
                    // token_type is TokenType::IllegalToken and char_type is TokenType::Literal(Literal::String(true)) -> token_type is TokenType::IllegalToken
                    TokenType::Literal(Literal::String(true)) => {
                        token_type = TokenType::IllegalToken;

                        match self.tokens.len() {
                            0 => {
                                self.tokens.push(Token {
                                    token_type: token_type,
                                    token_value: String::from(c.to_string()),
                                });
                            }
                            _ => {
                                self.tokens.last_mut().unwrap().token_type = token_type;
                                self.tokens.last_mut().unwrap().token_value = format!(
                                    "{}{}",
                                    self.tokens.last().unwrap().token_value,
                                    c.to_string()
                                );
                            }
                        }
                    }
                    // token_type is TokenType::IllegalToken and char_type isn't TokenType::Literal(Literal::String(true)) -> token_type is char_type
                    _ => {
                        token_type = char_type;
                        self.tokens.push(Token {
                            token_type: token_type,
                            token_value: String::from(c.to_string()),
                        });
                    }
                },
                _ => {
                    token_type = token_type;

                    self.tokens.last_mut().unwrap().token_type = token_type;
                    self.tokens.last_mut().unwrap().token_value = format!(
                        "{}{}",
                        self.tokens.last().unwrap().token_value,
                        c.to_string()
                    );
                }
            }

            last_token_type = token_type;
        }

        // Push Eof token at the end of the tokens vector
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            token_value: String::new(),
        });
    }
}

// One-character tokens checker
fn check_char_type(c: char) -> TokenType {
    match c {
        // '+'
        '+' => TokenType::Plus,
        // '-'
        '-' => TokenType::Minus,
        // '='
        '=' => TokenType::Eq,
        // '<'
        '<' => TokenType::Lt,
        // '>'
        '>' => TokenType::Gt,
        // '"'
        '"' => TokenType::Literal(Literal::String(false)),
        // From 0 to 9 digit
        c if c.is_digit(10) => TokenType::Literal(Literal::Integer),
        // Whitespace
        c if c.is_whitespace() => TokenType::Whitespace(1),
        // Alphanumeric character
        _ => TokenType::Literal(Literal::String(true)),
    }
}

// Multi-character token checker
fn check_token_type(char_type: TokenType, last_token_type: TokenType) -> TokenType {
    match char_type {
        TokenType::Eq => match last_token_type {
            // "=="
            TokenType::Eq => TokenType::EqEq,
            // "<="
            TokenType::Lt => TokenType::Le,
            // ">="
            TokenType::Gt => TokenType::Ge,
            _ => TokenType::IllegalToken,
        },
        TokenType::Literal(Literal::Integer) => match last_token_type {
            // Integer
            TokenType::Literal(Literal::Integer) => TokenType::Literal(Literal::Integer),
            // String
            TokenType::Literal(Literal::String(false)) => {
                TokenType::Literal(Literal::String(false))
            }
            _ => TokenType::IllegalToken,
        },
        TokenType::Literal(Literal::String(true)) => match last_token_type {
            TokenType::Literal(Literal::String(false)) => {
                TokenType::Literal(Literal::String(false))
            }
            _ => TokenType::IllegalToken,
        },
        TokenType::Literal(Literal::String(false)) => match last_token_type {
            TokenType::Literal(Literal::String(false)) => TokenType::Literal(Literal::String(true)),
            _ => TokenType::IllegalToken,
        },
        TokenType::Whitespace(1) => match last_token_type {
            // Whitespace
            TokenType::Whitespace(whitespace_count) => TokenType::Whitespace(whitespace_count + 1),
            _ => TokenType::IllegalToken,
        },
        _ => TokenType::IllegalToken,
    }
}
