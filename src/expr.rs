use crate::token::Token;
use crate::token::Literal;

pub(crate) trait Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}

pub(crate) trait Visitor<R> {
    fn visit_binary_expr<E: Expr>(&self, expr: &Binary<E>) -> R;
    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> R;
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> R;
}

pub(crate) struct Binary<E: Expr> {
    left: E,
    operator: Token,
    right: E,
}

impl<E: Expr> Expr for Binary<E> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_binary_expr(&self)
    }
}

pub(crate) struct Grouping<E: Expr> {
    expression: E,
}

impl<E: Expr> Expr for Grouping<E> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_grouping_expr(&self)
    }
}

pub(crate) struct LiteralExpr {
    value: Literal,
}

impl Expr for LiteralExpr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_literalexpr_expr(&self)
    }
}

pub(crate) struct Unary<E: Expr> {
    operator: Token,
    right: E,
}

impl<E: Expr> Expr for Unary<E> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_unary_expr(&self)
    }
}
