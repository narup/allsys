#![allow(dead_code)]

struct Token {
    token_type: TokenType,
    val: String,
}

#[derive(strum_macros::Display)]
enum TokenType {
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
    position: u16,      // current position in input (points to current char)
    read_position: u16, // current reading position in input, after current char
    ch: char,
}

impl Lexer {
    pub fn next_token(&self) -> String {
        String::from(&self.input)
    }
}

pub fn new(input: String) -> Lexer {
    Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: '\0',
    }
}

pub fn print_tokens() {
    let t1 = Token {
        token_type: TokenType::ILLEGAL,
        val: String::from("ILLEGAL"),
    };

    println!("First token: ({}, {})", t1.token_type.to_string(), t1.val);
}
