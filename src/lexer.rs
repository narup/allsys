#![allow(dead_code)]

static ILLEGAL: &'static str = "ILLEGAL";
static EOF: &'static str = "EOF";
static WHITESPACE: &'static str = "WHITESPACE";

//single character tokens
static PLUS: &'static str = "+";
static MINUS: &'static str = "-";
static MULTIPLY: &'static str = "*";
static DIVIDE: &'static str = "/";
static MODULO: &'static str = "%";
static LPAREN: &'static str = "(";
static RPAREN: &'static str = "(";
static LBRACKET: &'static str = "[";
static RBRACKET: &'static str = "]";
static DOT: &'static str = ".";
static LBRACE: &'static str = "{";
static RBRACE: &'static str = "}";
static COMMA: &'static str = ",";

//comparators
static GREATER_THAN: &'static str = ">";
static LESSER_THAN: &'static str = "<";
static EQ: &'static str = "==";
static NEQ: &'static str = "!=";
static GREATER_AND_EQ: &'static str = ">=";
static LESSER_AND_EQ: &'static str = "<=";

static ASSIGN: &'static str = "=";

//Keywords
static PRINT: &'static str = "print";
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
static CASE: &'static str = "case";
static NONE: &'static str = "none";
static COLON: &'static str = ":";
static CONTINUE: &'static str = "continue";
static BREAK: &'static str = "break";
static TRUE: &'static str = "true";
static FALSE: &'static str = "false";
static OR: &'static str = "or";
static AND: &'static str = "and";

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    val: &'static str,
    col: usize,
}

#[derive(strum_macros::Display, Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Whitespace,
    EndOfFile,

    //identifier + literals
    Identifier,
    Number,
    String,
    Assign,

    //Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    //comparators
    Equal,
    NotEqual,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqual,
    LesserThanOrEqual,

    //Delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    //Keywords
    Def,
    Defp,
    Let,
    Var,
    If,
    Else,
    ElsIf,
    For,
    Case,
    True,
    False,
    Or,
    And,
    None,
    Continue,
    Error,
    Handle,
    Raise,
}

pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    read_position: usize, // read position to look ahead
}

impl Lexer {
    pub fn parse(&mut self) -> Vec<Token> {
        println!("parsing the input: {}", self.input);

        let mut tokens = Vec::new();

        let mut token = self.next_token();
        while token.token_type != TokenType::EndOfFile {
            if matches!(token.token_type, TokenType::Whitespace) {
                token = self.next_token();
                continue;
            }
            println!("Next token: {:?}", token);
            tokens.push(token);
            token = self.next_token();
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        let next_char = self.read_char();
        match next_char {
            Some(c) => match c {
                ' ' | '\r' | '\n' | '\t' => {
                    return get_token(self.position, TokenType::Whitespace, WHITESPACE)
                }
                '\0' => return get_token(self.position, TokenType::EndOfFile, EOF),
                '+' => return get_token(self.position, TokenType::Plus, PLUS),
                '-' => return get_token(self.position, TokenType::Minus, MINUS),
                '*' => return get_token(self.position, TokenType::Multiply, MULTIPLY),
                '(' => return get_token(self.position, TokenType::LeftParen, LPAREN),
                ')' => return get_token(self.position, TokenType::RightParen, RPAREN),
                '=' => return get_token(self.position, TokenType::Assign, ASSIGN),
                _ => return self.get_complex_token(c),
            },
            None => return get_token(self.position, TokenType::Illegal, ILLEGAL),
        }
    }

    fn get_complex_token(&mut self, current_char: char) -> Token {
        let position = self.position - 1;
        if current_char.is_digit(10) {
            //handle digit
            while let Some(c) = self.peek_char() {
                if c.is_digit(10) {
                    self.read_char();
                } else {
                    break;
                }
            }

            let s:String = (&self.input[position..self.position]).to_string();
            return get_token(self.position, TokenType::Number, Box::leak(s.into_boxed_str()));
        }
        if current_char.is_alphabetic() {
            while let Some(c) = self.peek_char() {
                println!("current char: {}, next peek:{}", current_char, c);
                if c.is_alphabetic() {
                    self.read_char();
                } else {
                    break;
                }
            }

            println!("Reading char at {}...{}", position, self.position);

            let s:String = (&self.input[position..self.position]).to_string();
            return get_token(self.position, TokenType::Identifier, Box::leak(s.into_boxed_str()));
        }

        return get_token(self.position, TokenType::Illegal, ILLEGAL);
    }

    fn read_char(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return Some('\0');
        }

        let c = self.input.chars().nth(self.position);
        self.increment_position();

        return c;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return Some('\0');
        }
        self.input.chars().nth(self.read_position - 1)
    }

    fn increment_position(&mut self) {
        self.position = self.position + 1;
        self.read_position = self.position + 1;

        println!("Increment positions, current: {}, read: {}", self.position, self.read_position);
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
        let input = String::from("x = 2");

        let mut output_tokens: Vec<Token> = Vec::new();
        output_tokens.push(Token {
            token_type: TokenType::Identifier,
            val: "x",
            col: 1,
        });
        output_tokens.push(Token {
            token_type: TokenType::Assign,
            val: "=",
            col: 4,
        });

        output_tokens.push(Token {
            token_type: TokenType::Number,
            val: "2",
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
