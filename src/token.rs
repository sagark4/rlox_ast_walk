use crate::token_type::{TokenType, self};
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

impl Token {
    pub(crate) fn from(token_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Self{
       Token {
        token_type: token_type,
        lexeme: lexeme,
        literal: literal,
        line: line
       } 
    }
    
}