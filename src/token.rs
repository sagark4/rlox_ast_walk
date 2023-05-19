use crate::token_type::TokenType;
#[derive(Debug)]
pub(crate) enum Literal {
    Float(f64),
    StringLiteral(String),
}
#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Literal,
    pub(crate) line: i32,
}
