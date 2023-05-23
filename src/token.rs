use crate::token_type::TokenType;
#[derive(Clone, Debug)]
pub(crate) enum Literal {
    Float(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NoneLiteral,
}
impl Literal {
    pub (crate) fn is_truthy(&self) -> bool {
        match self {
            Literal::NoneLiteral => false,
            Literal::BoolLiteral(bool_val) => *bool_val,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Literal,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn from(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
