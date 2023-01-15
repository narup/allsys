use crate::token;

pub struct Parser {
    tokens: Vec<token::Token>,
    current_index: usize,
}

pub trait Expression {
    fn name(&self) -> String;
    fn value(&self) -> String;
}

pub struct UnaryExpression {
    token: Option<token::Token>,
    expr: Option<Box<dyn Expression>>,
}

impl Expression for UnaryExpression {
    fn name(&self) -> String {
        "unary".to_string()
    }

    fn value(&self) -> String {
        let expr_val = match &self.expr {
            Some(er) => er.value(),
            None => "".to_string(),
        };

        let opr = match self.token {
            Some(t) => t.val,
            None => "",
        };

        if expr_val == "" && opr == "" {
            return "val".to_string();
        }

        format!("{} {}", expr_val, opr)
    }
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

pub struct EmptyExpression {}

impl Expression for EmptyExpression {
    fn name(&self) -> String {
        "empty".to_string()
    }

    fn value(&self) -> String {
        "".to_string()
    }
}

pub struct LiteralExpression {
    token: Option<token::Token>,
}

impl Expression for LiteralExpression {
    fn name(&self) -> String {
        return "literal".to_string();
    }

    fn value(&self) -> String {
        let val = match self.token {
            Some(t) => t.val,
            None => "",
        };
        return val.to_string()
    }
}

impl Parser {
    pub fn parse(&mut self) -> Box<dyn Expression> {
        println!("Input tokens from the lexer:");
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
        let expr = self.comparision();

        let oprs = vec![token::TokenType::Equal, token::TokenType::NotEqual];
        self.build_expression(expr, &oprs, "comparision")
    }

    //comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparision(&mut self) -> Box<dyn Expression> {
        let expr = self.term();

        let oprs = vec![
            token::TokenType::GreaterThan,
            token::TokenType::GreaterThanOrEqual,
            token::TokenType::LesserThan,
            token::TokenType::LesserThanOrEqual,
        ];
        self.build_expression(expr, &oprs, "term")
    }

    //term  → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<dyn Expression> {
        let expr = self.factor();

        let oprs = vec![token::TokenType::Minus, token::TokenType::Plus];
        self.build_expression(expr, &oprs, "factor")
    }

    //factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Box<dyn Expression> {
        let expr = self.unary();

        let oprs = vec![token::TokenType::Divide, token::TokenType::Multiply];
        self.build_expression(expr, &oprs, "unary")
    }

    //unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Box<dyn Expression> {
        let oprs = vec![token::TokenType::Minus];
        if self.match_next_token(&oprs) {
            let opr = self.next_token();

            let expr = self.unary();

            return Box::new(UnaryExpression {
                expr: Some(expr),
                token: Some(opr),
            });
        }

        self.primary()
    }

    //primary → NUMBER | STRING | "true" | "false" | "nil"  | "(" expression ")" ;
    fn primary(&mut self) -> Box<dyn Expression> {
        let oprs = vec![
            token::TokenType::True,
            token::TokenType::False,
            token::TokenType::Number,
            token::TokenType::String,
            token::TokenType::Identifier,
        ];
        if self.match_next_token(&oprs) {
            let token = self.next_token();
            self.advance_token();
            return Box::new(LiteralExpression { token: Some(token) });
        }
        Box::new(BinaryExpression {
            left: None,
            right: None,
            token: None,
        })
    }

    fn build_expression(
        &mut self,
        expr: Box<dyn Expression>,
        oprs: &Vec<token::TokenType>,
        right_expr_type: &str,
    ) -> Box<dyn Expression> {
        let mut final_expr = expr;

        while self.match_next_token(oprs) {
            println!("found expression type {}", right_expr_type);
        
            let operator = self.next_token();

            self.advance_token();

            let mut right: Box<dyn Expression> = Box::new(EmptyExpression {});
            match right_expr_type {
                "comparision" => {
                    right = self.comparision();
                }
                "term" => {
                    right = self.term();
                }
                "factor" => {
                    right = self.factor();
                }
                "unary" => {
                    right = self.unary();
                }
                _ => println!("not supported"),
            }

            final_expr = Box::new(BinaryExpression {
                left: Some(final_expr),
                right: Some(right),
                token: Some(operator),
            });
        }

        final_expr
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
            if self.check_token(token_type) {
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
        let input = String::from("x >= y + 5");
        let mut lexer = lexer::new(input);
        let tokens = lexer.parse();

        let mut parser: Parser = parser::new(tokens);
        let expr = parser.parse();

        println!("Expression value: {}", expr.value());
    }
}
