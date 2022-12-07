#![allow(dead_code)]

static ILLEGAL: &'static str = "ILLEGAL";
static PLUS: &'static str = "+";
static MINUS: &'static str = "-";
static LPAREN: &'static str = "(";
static RPAREN: &'static str = "(";

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    val: &'static str,
    col: usize,
}

#[derive(strum_macros::Display, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //identifier + literals
    NUMBER,
    INTEGER,

    //Operators
    ASSIGN,
    PLUS,

    //Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keywords
    FUNCTION,
    LET,
}

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input, after current char
    ch: u8,
}

impl Lexer {
    pub fn parse(&mut self) -> Vec<Token> {
        let tokens = Vec::new();

        while self.position < 4 {
            self.read_char();
            self.increment_read_position();
        }

        tokens
    }

    pub fn next_token(&self) -> Token {
        let bytes = &self.input.as_bytes();

        println!("Loop over bytes");
        for (i, &item) in bytes.iter().enumerate() {
            match item {
                b'+' => return get_token(i, TokenType::PLUS, PLUS),
                _ => return get_token(i, TokenType::ILLEGAL, ILLEGAL),
            }
        }
        get_token(0, TokenType::ILLEGAL, ILLEGAL)
    }

    fn read_char(&self) -> Option<char> {
        let r = self.input.chars().nth(self.position);
        println!("Value of r: {:?}", r);
        r
    }

    fn increment_read_position(&mut self) {
        self.position = self.position + 1;
    }
}

fn get_token(col: usize, token_type: TokenType, val: &'static str) -> Token {
    Token {
        col,
        token_type,
        val,
    }
}

pub fn new(input: String) -> Lexer {
    Lexer {
        input: input.to_string(),
        position: 0,
        read_position: 0,
        ch: b' ',
    }
}

pub fn mod_name() -> String {
    "lexer".to_string()
}

//----- Tests ---

#[cfg(test)]
mod tests {
    use crate::lexer::mod_name;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("lexer", mod_name());
    }

    #[test]
    fn lexer_test() {
        let input = String::from("+(-)");

        let mut l = new(input);
        l.parse();

        println!("test passed")
    }
}
