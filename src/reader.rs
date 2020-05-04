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
        for c in self.src.chars() {
            let char_type: TokenType = check_char_type(c);

            let token_type: TokenType = check_token_type(char_type, last_token_type);

            match token_type {
                // Token is illegal -> Token is one-character token
                TokenType::IllegalToken => {
                    self.tokens.push(Token {
                        token_type: char_type,
                        token_value: String::from(c.to_string()),
                    });

                    last_token_type = char_type;
                }
                // Multi-character tokens handle
                _ => {
                    self.tokens.last_mut().unwrap().token_type = token_type;
                    self.tokens.last_mut().unwrap().token_value = format!(
                        "{}{}",
                        self.tokens.last().unwrap().token_value,
                        c.to_string()
                    );

                    last_token_type = token_type;
                }
            }
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
        // From 0 to 9 digit
        c if c.is_digit(10) => TokenType::Literal(Literal::Integer),
        c if c.is_whitespace() => TokenType::Whitespace,
        _ => TokenType::IllegalToken,
    }
}

// Multi-character token checker
fn check_token_type(char_type: TokenType, last_token_type: TokenType) -> TokenType {
    match char_type {
        TokenType::Eq => match last_token_type {
            // '=='
            TokenType::Eq => TokenType::EqEq,
            // '<='
            TokenType::Lt => TokenType::Le,
            // '>='
            TokenType::Gt => TokenType::Ge,
            _ => TokenType::IllegalToken,
        },
        TokenType::Literal(Literal::Integer) => match last_token_type {
            // Integer
            TokenType::Literal(Literal::Integer) => TokenType::Literal(Literal::Integer),
            _ => TokenType::IllegalToken,
        },
        _ => TokenType::IllegalToken,
    }
}
