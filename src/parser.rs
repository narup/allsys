
use crate::token;

pub struct Parser {
    tokens: Vec<token::Token>,
    current_index: usize
}

#[derive(PartialEq, Debug)]
pub struct Expression {

}

impl Parser {

    pub fn parse(&mut self) -> Expression {
        println!("Parsing tokens from the lexer:");
        for token in &self.tokens {
            println!("{:?}", token);
        }
        self.advance_token();
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        Expression {  }
    }

    fn advance_token(&mut self) {
        self.current_index = self.current_index + 1;
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
    use crate::parser;
    use crate::lexer;

    #[test]
    fn parser_expression_test() {
        let input = String::from("3 = 5");
        let mut lexer = lexer::new(input);
        let tokens = lexer.parse();

        let mut parser:Parser = parser::new(tokens);
        let expr:Expression = parser.parse();

        let expected = parser::Expression{};
        assert_eq!(expr, expected);
    }

}