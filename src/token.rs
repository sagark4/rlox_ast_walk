use crate::{token_type::TokenType, lox_callable::LoxCallable};
#[derive(Clone, Debug)]
pub(crate) enum Literal {
    Float(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NoneLiteral,
    Callable(LoxCallable)
}
impl Literal {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Self::NoneLiteral => false,
            Self::BoolLiteral(bool_val) => *bool_val,
            _ => true,
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

    pub(crate) fn negate_and_wrap(&self) -> Self {
        Self::BoolLiteral(!self.is_truthy())
    }

    pub(crate) fn unwrap_float(&self) -> f64 {
        match self {
            Self::Float(number) => *number,
            _ => panic!(),
        }
    }

    pub(crate) fn unwrap_str_literal(&self) -> &str {
        match self {
            Self::StringLiteral(str_literal) => &str_literal,
            _ => panic!(),
        }
    }

    pub(crate) fn wrap_float(value: f64) -> Self {
        Self::Float(value)
    }

    pub(crate) fn wrap_string_literal(value: String) -> Self {
        Self::StringLiteral(value)
    }

    pub(crate) fn wrap_bool(value: bool) -> Self {
        Self::BoolLiteral(value)
    }

    pub(crate) fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_string(&self) -> bool {
        match self {
            Self::StringLiteral(_) => true,
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
