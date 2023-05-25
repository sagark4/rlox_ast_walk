use crate::interpreter::RuntimeError;
use crate::token::Literal;
use crate::token::Token;
pub(crate) enum VisitorReturnType {
    VRString(String),
    VRLiteral(Literal),
    VRErr(RuntimeError),
}

impl VisitorReturnType {
    pub(crate) fn unwrap_negate_and_wrap_vrl_bool(&self) -> Self {
        match self {
            Self::VRLiteral(vrliteral) => {
                Self::VRLiteral(Literal::BoolLiteral(!vrliteral.is_truthy()))
            }
            _ => panic!(),
        }
    }

    pub(crate) fn unwrap_float(&self) -> f64 {
        match self {
            Self::VRLiteral(Literal::Float(number)) => *number,
            _ => panic!(),
        }
    }

    pub(crate) fn unwrap_str_literal(&self) -> &str {
        match self {
            Self::VRLiteral(Literal::StringLiteral(str_literal)) => &str_literal,
            _ => panic!(),
        }
    }

    pub(crate) fn wrap_float(value: f64) -> Self {
        Self::VRLiteral(Literal::Float(value))
    }

    pub(crate) fn wrap_string_literal(value: String) -> Self {
        Self::VRLiteral(Literal::StringLiteral(value))
    }

    pub(crate) fn wrap_bool(value: bool) -> Self {
        Self::VRLiteral(Literal::BoolLiteral(value))
    }

    pub(crate) fn is_float(&self) -> bool {
        match self {
            Self::VRLiteral(Literal::Float(_)) => true,
            _ => false,
        }
    }

    pub(crate) fn is_string(&self) -> bool {
        match self {
            Self::VRLiteral(Literal::StringLiteral(_)) => true,
            _ => false,
        }
    }

    pub(crate) fn is_vrl_equal_or_panic(&self, other: &Self) -> bool {
        if let Self::VRLiteral(fself) = self {
            if let Self::VRLiteral(fother) = other {
                return fself.is_equal(fother);
            }
        }
        panic!();
    }
}

pub(crate) trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnType;
}

pub(crate) trait Visitor {
    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnType;
    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnType;
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnType;
    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnType;
}

pub(crate) struct Binary {
    pub(crate) left: Box<dyn Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnType {
        visitor.visit_binary_expr(&self)
    }
}

impl Binary {
    pub(crate) fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

pub(crate) struct Grouping {
    pub(crate) expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnType {
        visitor.visit_grouping_expr(&self)
    }
}

impl Grouping {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct LiteralExpr {
    pub(crate) value: Literal,
}

impl Expr for LiteralExpr {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnType {
        visitor.visit_literalexpr_expr(&self)
    }
}

impl LiteralExpr {
    pub(crate) fn new(value: Literal) -> Box<Self> {
        Box::new(Self { value })
    }
}

pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnType {
        visitor.visit_unary_expr(&self)
    }
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { operator, right })
    }
}
