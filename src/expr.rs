use crate::token::Literal;
use crate::token::Token;

pub(crate) enum Expr {
    BinaryExpr(Box<Binary>),
    GroupingExpr(Box<Grouping>),
    LiteralExprExpr(Box<LiteralExpr>),
    UnaryExpr(Box<Unary>),
    VariableExpr(Box<Variable>),
    AssignExpr(Box<Assign>),
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::BinaryExpr(expr) => visitor.visit_binary_expr(expr),
            Expr::GroupingExpr(expr) => visitor.visit_grouping_expr(expr),
            Expr::LiteralExprExpr(expr) => visitor.visit_literalexpr_expr(expr),
            Expr::UnaryExpr(expr) => visitor.visit_unary_expr(expr),
            Expr::VariableExpr(expr) => visitor.visit_variable_expr(expr),
            Expr::AssignExpr(expr) => visitor.visit_assign_expr(expr),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literalexpr_expr(&mut self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
}

pub(crate) struct Binary {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl Binary {
    pub(crate) fn new(left: Expr, operator: Token, right: Expr) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

pub(crate) struct Grouping {
    pub(crate) expression: Expr,
}

impl Grouping {
    pub(crate) fn new(expression: Expr) -> Box<Self> {
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
    pub(crate) right: Expr,
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Expr) -> Box<Self> {
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

pub(crate) struct Assign {
    pub(crate) name: Token,
    pub(crate) value: Expr,
}

impl Assign {
    pub(crate) fn new(name: Token, value: Expr) -> Box<Self> {
        Box::new(Self { name, value })
    }
}
