use crate::token::Literal;
use crate::token::Token;

pub(crate) enum Expr {
    BinaryExpr(Box<Binary>),
    GroupingExpr(Box<Grouping>),
    LiteralExprExpr(Box<LiteralExpr>),
    UnaryExpr(Box<Unary>),
    VariableExpr(Box<Variable>),
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::BinaryExpr(expr) => visitor.visit_binary_expr(expr),
            Expr::GroupingExpr(expr) => visitor.visit_grouping_expr(expr),
            Expr::LiteralExprExpr(expr) => visitor.visit_literalexpr_expr(expr),
            Expr::UnaryExpr(expr) => visitor.visit_unary_expr(expr),
            Expr::VariableExpr(expr) => visitor.visit_variable_expr(expr),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
    fn visit_variable_expr(&self, expr: &Variable) -> R;
}

pub(crate) struct Binary {
    pub(crate) left: Box<Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

impl Binary {
    pub(crate) fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

pub(crate) struct Grouping {
    pub(crate) expression: Box<Expr>,
}

impl Grouping {
    pub(crate) fn new(expression: Box<Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct LiteralExpr {
    pub(crate) value: Literal,
}

impl LiteralExpr {
    pub(crate) fn new(value: Literal) -> Box<Self> {
        Box::new(Self { value })
    }
}

pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Box<Expr>) -> Box<Self> {
        Box::new(Self { operator, right })
    }
}

pub(crate) struct Variable {
    pub(crate) name: Token,
}

impl Variable {
    pub(crate) fn new(name: Token) -> Box<Self> {
        Box::new(Self { name })
    }
}
