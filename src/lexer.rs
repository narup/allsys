#![allow(dead_code)]

use std::collections::HashMap;
use crate::token;

pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    read_position: usize, // read position to look ahead
}

// contains the map of all the keywords
fn keyword_map() -> HashMap<&'static str, token::TokenType> {
    let mut map = HashMap::new();
    map.insert(token::VAR, token::TokenType::Var);
    map.insert(token::LET, token::TokenType::Let);
    map
}

impl Lexer {
    pub fn parse(&mut self) -> Vec<token::Token> {
        println!("parsing the input: {}", self.input);

        let mut tokens = Vec::new();
        while self.has_more_token() {
            let token = self.next_token();
            if matches!(token.token_type, token::TokenType::Whitespace) {
                continue;
            }
            if matches!(token.token_type, token::TokenType::Illegal) {
                print_error(format!("uncrecognized character '{}' at col {}", token.val, token.col).as_str());
                panic!("Exiting due to error");
            }
            println!("Next token: {:?}", token);
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> token::Token {
        let next_char = self.read_char();
        match next_char {
            Some(c) => match c {
                ' ' | '\r' | '\n' | '\t' => {
                    return self.get_token(token::TokenType::Whitespace, token::WHITESPACE)
                }
                '\0' => return self.get_token(token::TokenType::EndOfFile, token::EOF),
                '+' => return self.get_token(token::TokenType::Plus, token::PLUS),
                '-' => return self.get_token(token::TokenType::Minus, token::MINUS),
                '*' => return self.get_token(token::TokenType::Multiply, token::MULTIPLY),
                '/' => return self.get_token(token::TokenType::Divide, token::DIVIDE),
                '%' => return self.get_token(token::TokenType::Modulo, token::MODULO),
                '(' => return self.get_token(token::TokenType::LeftParen, token::LPAREN),
                ')' => return self.get_token(token::TokenType::RightParen, token::RPAREN),
                '{' => return self.get_token(token::TokenType::LeftBrace, token::LBRACE),
                '}' => return self.get_token(token::TokenType::RightBrace, token::RBRACE),
                '[' => return self.get_token(token::TokenType::LeftBracket, token::LBRACKET),
                ']' => return self.get_token(token::TokenType::RightBracket, token::RBRACKET),
                '=' => return self.get_token(token::TokenType::Assign, token::ASSIGN),
                ',' => return self.get_token(token::TokenType::Comma, token::COMMA),
                ':' => return self.get_token(token::TokenType::Colon, token::COLON),
                '>' => return self.get_token(token::TokenType::GreaterThan, token::GREATER_THAN),
                '<' => return self.get_token(token::TokenType::LesserThan, token::LESSER_THAN),
                _ => return self.get_complex_token(c),
            },
            None => return self.get_token(token::TokenType::Illegal, token::ILLEGAL),
        }
    }

    fn get_complex_token(&mut self, current_char: char) -> token::Token {
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
            return self.get_token(token::TokenType::Number, Box::leak(s.into_boxed_str()));
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
            return self.get_token(token::TokenType::Identifier, Box::leak(s.into_boxed_str()));
        }

        let s = String::from(current_char);
        return self.get_token(token::TokenType::Illegal, Box::leak(s.into_boxed_str()));
    }

    fn get_token(&mut self, token_type: token::TokenType, val: &'static str) -> token::Token {
        token::Token {
            col: self.position,
            token_type,
            val,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        if self.position > self.input.len() {
            return Some('\0');
        }

        let c = self.input.chars().nth(self.position);
        self.increment_position();

        return c;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position > self.input.len() {
            return Some('\0');
        }
        self.input.chars().nth(self.read_position - 1)
    }

    fn increment_position(&mut self) {
        self.position = self.position + 1;
        self.read_position = self.position + 1;

        println!("Increment positions, current: {}, read: {}", self.position, self.read_position);
    }

    fn has_more_token(&self) -> bool {
        self.position < self.input.len()
    }
}

fn print_error(err: &str) {
    println!("ERROR:{}", err);
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

    use std::collections::HashMap;

    use super::*;

    // contains the map of all the keywords
    fn test_cases_map() -> HashMap<String, Vec<token::Token>> {
        let mut map = HashMap::new();
        map.insert("x = 2".to_string(), test_tokens_1());
        map
    }

    #[test]
    fn it_works() {
        assert_eq!("lexer", mod_name());
    }

    #[test]
    fn test_1() {
        for (k, v) in test_cases_map() {
            let l = new(k);
            lexer_test(l, v);
        }
    }

    fn lexer_test(mut l:Lexer, expected_tokens: Vec<token::Token>) {
        let tokens = l.parse();
        for (index, t) in tokens.iter().enumerate() {
            let expected_t: Option<&token::Token> = expected_tokens.get(index);
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

    fn test_tokens_1() -> Vec<token::Token>{
        let mut output_tokens: Vec<token::Token> = Vec::new();
        output_tokens.push(token::Token {
            token_type: token::TokenType::Identifier,
            val: "x",
            col: 1,
        });
        output_tokens.push(token::Token {
            token_type: token::TokenType::Assign,
            val: "=",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::Number,
            val: "2",
            col: 4,
        });

        output_tokens
    }
}
