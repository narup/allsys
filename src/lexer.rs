#![allow(dead_code)]

static ILLEGAL: &'static str = "ILLEGAL";
static WHITESPACE: &'static str = "WHITESPACE";

//operators
static PLUS: &'static str = "+";
static MINUS: &'static str = "-";
static MULTIPLY: &'static str = "*";
static DIVIDE: &'static str = "/";
static MODULO: &'static str = "%";
static LPAREN: &'static str = "(";
static RPAREN: &'static str = "(";
static LBRACKET: &'static str = "[";
static RBRACKET: &'static str = "]";

//comparators
static GREATER_THAN: &'static str = ">";
static LESSER_THAN: &'static str = "<";
static EQ: &'static str = "==";
static NEQ: &'static str = "!=";
static GREATER_AND_EQ: &'static str = ">=";
static LESSER_AND_EQ: &'static str = "<=";

static ASSIGN: &'static str = "=";

//Keywords
static LET: &'static str = "let";
static VAR: &'static str = "var";
static DEF: &'static str = "def";
static DEFP: &'static str = "defp";
static MODULE: &'static str = "module";
static FOR: &'static str = "for";
static IF: &'static str = "if";
static ELSE: &'static str = "else";
static ELSIF: &'static str = "elsif";
static RAISE: &'static str = "raise";
static ERROR: &'static str = "error";
static HANDLE: &'static str = "handle";
static LBRACE: &'static str = "{";
static RBRACE: &'static str = "}";
static COMMA: &'static str = ",";
static CASE: &'static str = "case";
static NONE: &'static str = "none";
static COLON: &'static str = ":";
static CONTINUE: &'static str = "continue";
static BREAK: &'static str = "break";

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    val: &'static str,
    col: usize,
}

#[derive(strum_macros::Display, Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    WHITESPACE,
    EOF,

    //identifier + literals
    NUMBER,
    INTEGER,

    ASSIGN,

    //Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    MODULO,

    //comparators
    EQ,
    NEQ,
    GREATER_THAN,
    LESSER_THAN,
    GREATER_AND_EQ,
    LESSER_AND_EQ,

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
    position: usize, // current position in input (points to current char)
    ch: u8,
}

impl Lexer {
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let token = self.next_token();
            self.increment_position();

            if matches!(token.token_type, TokenType::WHITESPACE) {
                continue;
            }

            tokens.push(token);
        }

        tokens
    }

    fn next_token(&self) -> Token {
        let next_char = self.input.chars().nth(self.position);
        match next_char {
            Some(c) => match c {
                ' ' | '\r' | '\n' | '\t' => {
                    return get_token(self.position, TokenType::WHITESPACE, WHITESPACE)
                }
                '+' => return get_token(self.position, TokenType::PLUS, PLUS),
                '-' => return get_token(self.position, TokenType::MINUS, MINUS),
                '*' => return get_token(self.position, TokenType::MINUS, MINUS),
                '(' => return get_token(self.position, TokenType::LPAREN, LPAREN),
                ')' => return get_token(self.position, TokenType::RPAREN, RPAREN),
                _ => return get_token(self.position, TokenType::ILLEGAL, ILLEGAL),
            },
            None => return get_token(self.position, TokenType::ILLEGAL, ILLEGAL),
        }
    }

    fn increment_position(&mut self) {
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
        let input = String::from("+(");

        let mut output_tokens: Vec<Token> = Vec::new();
        output_tokens.push(Token {
            token_type: TokenType::PLUS,
            val: "+",
            col: 4,
        });
        output_tokens.push(Token {
            token_type: TokenType::LPAREN,
            val: "(",
            col: 4,
        });

        let mut l = new(input);
        let tokens = l.parse();
        for (index, t) in tokens.iter().enumerate() {
            let expected_t: Option<&Token> = output_tokens.get(index);
            match expected_t {
                Some(expected_t) => {
                    assert_eq!(expected_t.val, t.val);
                    assert_eq!(expected_t.token_type, t.token_type);
                }
                None => assert!(false),
            }
        }

        println!("test passed");
    }
}
