#![allow(dead_code)]

pub static ILLEGAL: &'static str = "ILLEGAL";
pub static EOF: &'static str = "EOF";
pub static WHITESPACE: &'static str = "WHITESPACE";
pub static NEWLINE: &'static str = "NEWLINE";
//single character tokens
pub static PLUS: &'static str = "+";
pub static MINUS: &'static str = "-";
pub static MULTIPLY: &'static str = "*";
pub static DIVIDE: &'static str = "/";
pub static MODULO: &'static str = "%";
pub static LPAREN: &'static str = "(";
pub static RPAREN: &'static str = ")";
pub static LBRACKET: &'static str = "[";
pub static RBRACKET: &'static str = "]";
pub static DOT: &'static str = ".";
pub static LBRACE: &'static str = "{";
pub static RBRACE: &'static str = "}";
pub static COMMA: &'static str = ",";
pub static COLON: &'static str = ":";
pub static COMMENT: &'static str = "//";
//comparators
pub static GREATER_THAN: &'static str = ">";
pub static LESSER_THAN: &'static str = "<";
pub static EQ: &'static str = "==";
pub static NEQ: &'static str = "!=";
pub static GREATER_AND_EQ: &'static str = ">=";
pub static LESSER_AND_EQ: &'static str = "<=";
pub static ASSIGN: &'static str = "=";
//Keywords
pub static PRINT: &'static str = "print";
pub static LET: &'static str = "let";
pub static VAR: &'static str = "var";
pub static DEF: &'static str = "def";
pub static DEFP: &'static str = "defp";
pub static MODULE: &'static str = "module";
pub static FOR: &'static str = "for";
pub static IF: &'static str = "if";
pub static ELSE: &'static str = "else";
pub static ELSIF: &'static str = "elsif";
pub static RAISE: &'static str = "raise";
pub static UNTERMINATED_STRING: &'static str = "unterminated string";
pub static ERROR: &'static str = "error";
pub static HANDLE: &'static str = "handle";
pub static CASE: &'static str = "case";
pub static NONE: &'static str = "none";
pub static CONTINUE: &'static str = "continue";
pub static BREAK: &'static str = "break";
pub static TRUE: &'static str = "true";
pub static FALSE: &'static str = "false";
pub static OR: &'static str = "||";
pub static AND: &'static str = "&&";

pub static IDENT: &'static str = "IDENT";
pub static NUMBER: &'static str = "NUMBER";
pub static STRING: &'static str = "STRING";

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub val: &'static str,
    pub col: usize,
}

#[derive(strum_macros::Display, Debug, PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum TokenType {
    Illegal,
    Whitespace,
    Newline,
    UnterminatedString,
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
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comment,
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

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Illegal => ILLEGAL,
            TokenType::Whitespace => WHITESPACE,
            TokenType::Newline => NEWLINE,
            TokenType::EndOfFile => EOF,
            TokenType::Identifier => IDENT,
            TokenType::Number => NUMBER,
            TokenType::String => STRING,
            TokenType::Assign => ASSIGN,
            TokenType::Plus => PLUS,
            TokenType::Minus => MINUS,
            TokenType::Multiply => MULTIPLY,
            TokenType::Divide => DIVIDE,
            TokenType::Modulo => MODULO,
            TokenType::Equal => EQ,
            TokenType::NotEqual => NEQ,
            TokenType::GreaterThan => GREATER_THAN,
            TokenType::LesserThan => LESSER_THAN,
            TokenType::GreaterThanOrEqual => GREATER_AND_EQ,
            TokenType::LesserThanOrEqual => LESSER_AND_EQ,
            TokenType::Comma => COMMA,
            TokenType::Colon => COLON,
            TokenType::LeftParen => LPAREN,
            TokenType::RightParen => RPAREN,
            TokenType::LeftBrace => LBRACE,
            TokenType::RightBrace => RBRACE,
            TokenType::LeftBracket => LBRACKET,
            TokenType::RightBracket => RBRACKET,
            TokenType::Comment => COMMENT,
            TokenType::Def => DEF,
            TokenType::Defp => DEFP,
            TokenType::Let => LET,
            TokenType::Var => VAR,
            TokenType::If => IF,
            TokenType::Else => ELSE,
            TokenType::ElsIf => ELSIF,
            TokenType::For => FOR,
            TokenType::Case => CASE,
            TokenType::True => TRUE,
            TokenType::False => FALSE,
            TokenType::Or => OR,
            TokenType::And => AND,
            TokenType::None => NONE,
            TokenType::Continue => CONTINUE,
            TokenType::Error => ERROR,
            TokenType::Handle => HANDLE,
            TokenType::Raise => RAISE,
            TokenType::UnterminatedString => UNTERMINATED_STRING,
        }
    }
}