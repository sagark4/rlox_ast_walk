use crate::token::Token;
use crate::token::Literal;

pub(crate) trait Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}

pub(crate) trait Visitor<R> {
    fn visit_binary_expr<D: Expr, U: Expr>(&self, expr: &Binary<D, U>) -> R;
    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> R;
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> R;
}

pub(crate) struct Binary<L, R> {
    pub(crate)  left: L,
    pub(crate)  operator: Token,
    pub(crate)  right: R,
}

impl<L: Expr, R: Expr> Expr for Binary<L, R> {
    fn accept<A>(&self, visitor: &impl Visitor<A>) -> A {
        visitor.visit_binary_expr(&self)
    }
}

impl<L: Expr, R: Expr> Binary<L, R> {
    pub(crate) fn new(left: L, operator: Token, right: R, ) -> Self{
        Self {left, operator, right, }
    }
}

pub(crate) struct Grouping<E: Expr> {
    pub(crate)  expression: E,
}

impl<E: Expr> Expr for Grouping<E> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_grouping_expr(&self)
    }
}

impl<E: Expr> Grouping<E> {
    pub(crate) fn new(expression: E, ) -> Self{
        Self {expression, }
    }
}

pub(crate) struct LiteralExpr {
    pub(crate)  value: Literal,
}

impl Expr for LiteralExpr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_literalexpr_expr(&self)
    }
}

impl LiteralExpr {
    pub(crate) fn new(value: Literal, ) -> Self{
        Self {value, }
    }
}

pub(crate) struct Unary<E: Expr> {
    pub(crate)  operator: Token,
    pub(crate)  right: E,
}

impl<E: Expr> Expr for Unary<E> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_unary_expr(&self)
    }
}

impl<E: Expr> Unary<E> {
    pub(crate) fn new(operator: Token, right: E, ) -> Self{
        Self {operator, right, }
    }
}
