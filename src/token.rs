use crate::token_type::TokenType;
#[derive(Clone, Debug)]
pub(crate) enum Literal {
    Float(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NoneLiteral,
}
impl Literal {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Self::NoneLiteral => false,
            Self::BoolLiteral(bool_val) => *bool_val,
            _ => false,
        }
    }

    pub(crate) fn is_equal(&self, other: &Self) -> bool {
        if let Self::NoneLiteral = self {
            if let Self::NoneLiteral = other {
                return true;
            }
        }
        if let Self::Float(fself) = self {
            if let Self::Float(fother) = other {
                return fself == fother;
            }
        }
        if let Self::BoolLiteral(bself) = self {
            if let Self::BoolLiteral(bother) = other {
                return bself == bother;
            }
        }
        if let Self::StringLiteral(sself) = self {
            if let Self::StringLiteral(sother) = other {
                return sself == sother;
            }
        }
        return false;
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
