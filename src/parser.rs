use crate::token;

pub struct Parser {
    tokens: Vec<token::Token>,
    current_index: usize,
}

pub trait Expression {
    fn name(&self) -> String;
    fn value(&self) -> String;
}

pub struct BinaryExpression {
    left: Option<Box<dyn Expression>>,
    right: Option<Box<dyn Expression>>,
    token: Option<token::Token>,
}

impl Expression for BinaryExpression {
    fn name(&self) -> String {
        return "binary".to_string();
    }

    fn value(&self) -> String {
        println!("Printing the value");

        let left_val = match &self.left {
            Some(l) => l.value(),
            None => "".to_string(),
        };

        let right_val = match &self.right {
            Some(r) => r.value(),
            None => "".to_string(),
        };

        let opr = match self.token {
            Some(t) => t.val,
            None => "",
        };

        if left_val == "" && right_val == "" && opr == "" {
            return "val".to_string();
        }

        format!("{} {} {}", left_val, opr, right_val)
    }
}

impl Parser {
    pub fn parse(&mut self) -> Box<dyn Expression> {
        println!("Parsing tokens from the lexer:");
        for token in &self.tokens {
            println!("{:?}", token);
        }
        self.expression()
    }

    // expression -> equality
    fn expression(&mut self) -> Box<dyn Expression> {
        return self.equality();
    }

    //equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparision();
        self.advance_token();

        let term_oprs = vec![token::TokenType::Equal, token::TokenType::NotEqual];

        while self.match_next_token(&term_oprs) {
            let operator = self.next_token();

            println!("TOKEN: {:?}", operator);

            self.advance_token();

            let right = self.comparision();

            expr = Box::new(BinaryExpression {
                left: Some(expr),
                right: Some(right),
                token: Some(operator),
            });
        }

        expr
    }

    //comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparision(&mut self) -> Box<dyn Expression> {
        Box::new(BinaryExpression {
            left: None,
            right: None,
            token: None,
        })
    }

    fn advance_token(&mut self) {
        self.current_index = self.current_index + 1;
        println!("Next token index: {}", self.current_index);
    }

    fn next_token(&mut self) -> token::Token {
        let token = self.tokens.get(self.current_index);
        match token {
            Some(t) => return *t,
            None => token::new_illegal_token(),
        }
    }

    fn match_next_token(&mut self, match_tokens: &Vec<token::TokenType>) -> bool {
        for token_type in match_tokens {
            if self.check_token(&token_type) {
                return true;
            }
        }

        return false;
    }

    fn check_token(&mut self, next_token: &token::TokenType) -> bool {
        let token = self.tokens.get(self.current_index);
        match token {
            Some(t) => {
                if t.token_type.as_str() == next_token.as_str() {
                    return true;
                }
            }
            None => return false,
        }

        return false;
    }
}

pub fn new(tokens: Vec<token::Token>) -> Parser {
    Parser {
        tokens,
        current_index: 0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn parser_expression_test() {
        let input = String::from("3 == != 5");
        let mut lexer = lexer::new(input);
        let tokens = lexer.parse();

        let mut parser: Parser = parser::new(tokens);
        let expr = parser.parse();

        println!("Expression value: {}", expr.value());
    }
}
