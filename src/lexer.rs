#![allow(dead_code)]

use crate::token;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, token::TokenType> = {
        let mut map = HashMap::new();
        map.insert(token::MODULE, token::TokenType::Module);
        map.insert(token::PRINT, token::TokenType::Print);
        map.insert(token::VAR, token::TokenType::Var);
        map.insert(token::LET, token::TokenType::Let);
        map.insert(token::DEF, token::TokenType::Def);
        map.insert(token::DEFP, token::TokenType::Defp);
        map.insert(token::IF, token::TokenType::If);
        map.insert(token::ELSE, token::TokenType::Else);
        map.insert(token::ELSIF, token::TokenType::ElsIf);
        map.insert(token::FOR, token::TokenType::For);
        map.insert(token::CASE, token::TokenType::Case);
        map.insert(token::CONTINUE, token::TokenType::Continue);
        map.insert(token::NONE, token::TokenType::None);
        map.insert(token::TRUE, token::TokenType::True);
        map.insert(token::FALSE, token::TokenType::False);
        map.insert(token::AND, token::TokenType::And);
        map.insert(token::OR, token::TokenType::Or);
        map.insert(token::ERROR, token::TokenType::Error);
        map.insert(token::HANDLE, token::TokenType::Handle);
        map.insert(token::RAISE, token::TokenType::Raise);

        map
    };
}

pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    line: usize,
    read_position: usize, // read position to look ahead
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
                    self.increment_position();
                    continue;
                }
            }
            if matches!(token.token_type, token::TokenType::Illegal) {
                print_error(
                    format!(
                        "uncrecognized character '{}' at col {}, line {}",
                        token.val, token.col, self.line
                    )
                    .as_str(),
                );
                println!("^^^exiting program execution^^^");
                process::exit(1);
            }
            if matches!(token.token_type, token::TokenType::UnterminatedString) {
                print_error(
                    format!(
                        "Unterminated string '{}...' at col {}, line {}",
                        token.val, token.col, self.line
                    )
                    .as_str(),
                );
                println!("^^^exiting program execution^^^");
                process::exit(1);
            }
            if matches!(token.token_type, token::TokenType::InvalidNumber) {
                print_error(
                    format!(
                        "Invalid number input '{}...' at col {}, line {}",
                        token.val, token.col, self.line
                    )
                    .as_str(),
                );
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
                ' ' | '\r' | '\t' => return self.single_char_token(token::TokenType::Whitespace),
                '\n' => {
                    self.line = self.line + 1;
                    return self.single_char_token(token::TokenType::Newline);
                }
                '\0' => return self.single_char_token(token::TokenType::EndOfFile),
                '+' => return self.single_char_token(token::TokenType::Plus),
                '-' => return self.single_char_token(token::TokenType::Minus),
                '*' => return self.single_char_token(token::TokenType::Multiply),
                '/' => {
                    return self.multi_char_token(
                        '/',
                        token::TokenType::Comment,
                        token::TokenType::Divide,
                    )
                }
                '%' => return self.single_char_token(token::TokenType::Modulo),
                '(' => return self.single_char_token(token::TokenType::LeftParen),
                ')' => return self.single_char_token(token::TokenType::RightParen),
                '{' => return self.single_char_token(token::TokenType::LeftBrace),
                '}' => return self.single_char_token(token::TokenType::RightBrace),
                '[' => return self.single_char_token(token::TokenType::LeftBracket),
                ']' => return self.single_char_token(token::TokenType::RightBracket),
                ',' => return self.single_char_token(token::TokenType::Comma),
                ':' => return self.single_char_token(token::TokenType::Colon),
                '!' => {
                    return self.multi_char_token(
                        '=',
                        token::TokenType::NotEqual,
                        token::TokenType::Illegal,
                    )
                }
                '=' => {
                    return self.multi_char_token(
                        '=',
                        token::TokenType::Equal,
                        token::TokenType::Assign,
                    )
                }
                '>' => {
                    return self.multi_char_token(
                        '=',
                        token::TokenType::GreaterThanOrEqual,
                        token::TokenType::GreaterThan,
                    )
                }
                '<' => {
                    return self.multi_char_token(
                        '=',
                        token::TokenType::LesserThanOrEqual,
                        token::TokenType::LesserThan,
                    )
                }
                '&' => {
                    return self.multi_char_token(
                        '&',
                        token::TokenType::And,
                        token::TokenType::Illegal,
                    )
                }
                '|' => {
                    return self.multi_char_token(
                        '|',
                        token::TokenType::Or,
                        token::TokenType::Illegal,
                    )
                }
                '"' => return self.get_string_token(),
                _ => return self.get_complex_token(c),
            },
            None => return self.single_char_token(token::TokenType::Illegal),
        }
    }

    fn multi_char_token(
        &mut self,
        expected_char: char,
        expected_token: token::TokenType,
        default_token: token::TokenType,
    ) -> token::Token {
        if self.match_next_char(expected_char) {
            self.increment_position();
            self.single_char_token(expected_token)
        } else {
            self.single_char_token(default_token)
        }
    }

    fn get_string_token(&mut self) -> token::Token {
        let position = self.position;
        while !self.match_next_char('"') && self.has_more_token() {
            self.increment_position();
        }

        if !self.has_more_token() {
            let mut end_position = self.position;
            if end_position > 30 {
                end_position = 30;
            }
            let s: String = (&self.input[position..end_position]).to_string();
            return self.get_token_with_val(
                token::TokenType::UnterminatedString,
                Box::leak(s.into_boxed_str()),
            );
        }

        //skip ending '"'
        self.increment_position();

        // Trim the surrounding quotes
        let s: String = (&self.input[position..self.position - 1]).to_string();
        return self.get_token_with_val(token::TokenType::String, Box::leak(s.into_boxed_str()));
    }

    fn get_complex_token(&mut self, current_char: char) -> token::Token {
        let position = self.position - 1;
        if current_char.is_digit(10) {
            //handle digit
            while self.peek_char().is_digit(10) {
                self.read_char();
            }
            if self.peek_char() == '.' && self.peek_next_char().is_digit(10) {
                self.read_char();
                while self.peek_char().is_digit(10) {
                    self.read_char();
                }
            }

            let s: String = (&self.input[position..self.position]).to_string();
            return self
                .get_token_with_val(token::TokenType::Number, Box::leak(s.into_boxed_str()));
        }
        if current_char.is_alphanumeric() {
            while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
                self.read_char();
            }

            let s: String = (&self.input[position..self.position]).to_string();
            let token_str: &str = &s;
            let token = KEYWORDS.get(&token_str);
            match token {
                Some(t) => return self.get_token_with_val(*t, Box::leak(s.into_boxed_str())),
                None => {
                    return self.get_token_with_val(
                        token::TokenType::Identifier,
                        Box::leak(s.into_boxed_str()),
                    )
                }
            }
        }

        let s = String::from(current_char);
        return self.get_token_with_val(token::TokenType::Illegal, Box::leak(s.into_boxed_str()));
    }

    fn single_char_token(&mut self, token_type: token::TokenType) -> token::Token {
        let val = token_type.as_str();
        return self.get_token_with_val(token_type, val);
    }

    fn get_token_with_val(
        &mut self,
        token_type: token::TokenType,
        val: &'static str,
    ) -> token::Token {
        token::Token {
            col: self.position,
            token_type,
            val,
        }
    }

    fn match_next_char(&mut self, expected_char: char) -> bool {
        let next_char = self.peek_char();
        if expected_char == next_char {
            return true;
        } else {
            return false;
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

    fn peek_char(&mut self) -> char {
        if (self.read_position - 1) > self.input.len() {
            return '\0';
        }
        match self.input.chars().nth(self.read_position - 1) {
            Some(c) => return c,
            None => return '\0',
        }
    }

    fn peek_next_char(&mut self) -> char {
        if self.read_position > self.input.len() {
            return '\0';
        }
        match self.input.chars().nth(self.read_position) {
            Some(c) => return c,
            None => return '\0',
        }
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
        line: 1,
    }
}

pub fn mod_name() -> String {
    "lexer".to_string()
}

//----- Tests ---

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    // contains the map of all the keywords
    fn test_cases_map() -> HashMap<String, Vec<token::Token>> {
        let mut map = HashMap::new();
        map.insert("x = 2 //this is puran\n".to_string(), test_tokens_1());
        map.insert("val == 52.50 && y != 200".to_string(), test_tokens_2());
        map.insert("y == \"this is my string\"".to_string(), test_tokens_3());
        map.insert("let x = \"test\"".to_string(), test_tokens_4());

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

    fn lexer_test(mut l: Lexer, expected_tokens: Vec<token::Token>) {
        println!("testing testing testn");
        let tokens = l.parse();
        for (index, t) in tokens.iter().enumerate() {
            let expected_t: Option<&token::Token> = expected_tokens.get(index);
            match expected_t {
                Some(expected_t) => {
                    println!("returned token: {:?}", t);
                    assert_eq!(expected_t.val, t.val);
                    assert_eq!(expected_t.token_type, t.token_type);
                }
                None => {
                    println!("failed expected assertion on {:?}", t);
                    assert!(false);
                }
            }
        }

        println!("test passed");
    }

    fn test_tokens_1() -> Vec<token::Token> {
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

        output_tokens.push(token::Token {
            token_type: token::TokenType::Newline,
            val: "NEWLINE",
            col: 4,
        });

        output_tokens
    }

    fn test_tokens_2() -> Vec<token::Token> {
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
            val: "52.50",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::And,
            val: "&&",
            col: 4,
        });

        output_tokens.push(token::Token {
            token_type: token::TokenType::Identifier,
            val: "y",
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

fn test_tokens_3() -> Vec<token::Token> {
    let mut output_tokens: Vec<token::Token> = Vec::new();
    output_tokens.push(token::Token {
        token_type: token::TokenType::Identifier,
        val: "y",
        col: 1,
    });

    output_tokens.push(token::Token {
        token_type: token::TokenType::Equal,
        val: "==",
        col: 1,
    });

    output_tokens.push(token::Token {
        token_type: token::TokenType::String,
        val: "this is my string",
        col: 1,
    });

    return output_tokens;
}

fn test_tokens_4() -> Vec<token::Token> {
    let mut output_tokens: Vec<token::Token> = Vec::new();
    output_tokens.push(token::Token {
        token_type: token::TokenType::Let,
        val: "let",
        col: 1,
    });

    output_tokens.push(token::Token {
        token_type: token::TokenType::Identifier,
        val: "x",
        col: 1,
    });

    output_tokens.push(token::Token {
        token_type: token::TokenType::Assign,
        val: "=",
        col: 1,
    });

    output_tokens.push(token::Token {
        token_type: token::TokenType::String,
        val: "test",
        col: 1,
    });

    return output_tokens;
}
