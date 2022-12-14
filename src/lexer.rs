#![allow(dead_code)]

use std::collections::HashMap;
use std::process;
use crate::token;

pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    line: usize,
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
            if matches!(token.token_type, token::TokenType::Comment) {
                //for comment token we just consume the rest of the line
                while !self.match_next_char('\n') {
                    continue;
                }
            }
            if matches!(token.token_type, token::TokenType::Illegal) {
                print_error(format!("uncrecognized character '{}' at col {}", token.val, token.col).as_str());
                println!("^^^exiting program execution^^^");
                process::exit(1);
            }
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> token::Token {
        let next_char = self.read_char();
        match next_char {
            Some(c) => match c {
                ' ' | '\r' | '\t' => {
                    return self.single_char_token(token::TokenType::Whitespace)
                }
                '\n' => {
                    self.line = self.line + 1;
                    return self.single_char_token(token::TokenType::Whitespace)
                }
                '\0' => return self.single_char_token(token::TokenType::EndOfFile),
                '+' => return self.single_char_token(token::TokenType::Plus),
                '-' => return self.single_char_token(token::TokenType::Minus),
                '*' => return self.single_char_token(token::TokenType::Multiply),
                '/' => return self.multi_char_token('/', token::TokenType::Comment, token::TokenType::Divide),
                '%' => return self.single_char_token(token::TokenType::Modulo),
                '(' => return self.single_char_token(token::TokenType::LeftParen),
                ')' => return self.single_char_token(token::TokenType::RightParen),
                '{' => return self.single_char_token(token::TokenType::LeftBrace),
                '}' => return self.single_char_token(token::TokenType::RightBrace),
                '[' => return self.single_char_token(token::TokenType::LeftBracket),
                ']' => return self.single_char_token(token::TokenType::RightBracket),
                ',' => return self.single_char_token(token::TokenType::Comma),
                ':' => return self.single_char_token(token::TokenType::Colon),
                '!' => return self.multi_char_token('=', token::TokenType::NotEqual, token::TokenType::Illegal),
                '=' => return self.multi_char_token('=', token::TokenType::Equal, token::TokenType::Assign),
                '>' => return self.multi_char_token('=', token::TokenType::GreaterThanOrEqual, token::TokenType::GreaterThan),
                '<' => return self.multi_char_token('=', token::TokenType::LesserThanOrEqual, token::TokenType::LesserThan),
                '&' => return self.multi_char_token('&', token::TokenType::And, token::TokenType::Illegal),
                '|' => return self.multi_char_token('|', token::TokenType::Or, token::TokenType::Illegal),
                _ => return self.get_complex_token(c),
            },
            None => return self.single_char_token(token::TokenType::Illegal),
        }
    }

    fn multi_char_token(&mut self, expected_char: char, expected_token: token::TokenType, default_token: token::TokenType) -> token::Token {
        if self.match_next_char(expected_char) {
            self.single_char_token(expected_token)
        } else {
            self.single_char_token(default_token)
        }
    }

    fn match_next_char(&mut self, expected_char: char) -> bool {
        let next_char = self.read_char();
        match next_char {
            Some(c) => {
                if expected_char == c {
                    return true
                } else {
                    return false
                }
            }
            None => return false
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
            return self.get_token_with_val(token::TokenType::Number, Box::leak(s.into_boxed_str()));
        }
        if current_char.is_alphabetic() {
            while let Some(c) = self.peek_char() {
                if c.is_alphabetic() {
                    self.read_char();
                } else {
                    break;
                }
            }

            let s:String = (&self.input[position..self.position]).to_string();
            return self.get_token_with_val(token::TokenType::Identifier, Box::leak(s.into_boxed_str()));
        }

        let s = String::from(current_char);
        return self.get_token_with_val(token::TokenType::Illegal, Box::leak(s.into_boxed_str()));
    }

    fn single_char_token(&mut self, token_type: token::TokenType) -> token::Token {
        let val = token_type.as_str();
        return self.get_token_with_val(token_type, val);
    }

    fn get_token_with_val(&mut self, token_type: token::TokenType, val: &'static str) -> token::Token {
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
        line: 0,
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
        map.insert("x = 2 //this is puran\n".to_string(), test_tokens_1());
        map.insert("val == 5 && val != 200".to_string(), test_tokens_2());
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

        output_tokens.push(token::Token {
            token_type: token::TokenType::Comment,
            val: "//",
            col: 4,
        });

        output_tokens
    }

fn test_tokens_2() -> Vec<token::Token>{
        let mut output_tokens: Vec<token::Token> = Vec::new();
        output_tokens.push(token::Token {
            token_type: token::TokenType::Identifier,
            val: "val",
            col: 1,
        });
        output_tokens.push(token::Token {
            token_type: token::TokenType::Equal,
            val: "==",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::Number,
            val: "5",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::And,
            val: "&&",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::Identifier,
            val: "val",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::NotEqual,
            val: "!=",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::Number,
            val: "200",
            col: 4,
        });

        output_tokens
    }
}
