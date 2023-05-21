use crate::expr::*;
use crate::token::Literal::NoneLiteral;
pub(crate) struct AstPrinter {}

impl AstPrinter {
    pub(crate) fn print(&self, expr: &impl Expr) -> String {
        expr.accept::<String>(self)
    }
    // I cannot put these two expressions in a vector because they are not object safe; so I have to prepare as many functions as there are children expressions!
    fn parenthesize_two<L: Expr, R: Expr>(&self, name: &str, lexpr: &L, rexpr: &R) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(&lexpr.accept::<String>(self));
        builder.push(' ');
        builder.push_str(name);
        builder.push(' ');
        builder.push_str(&rexpr.accept::<String>(self));
        builder.push(')');
        builder
    }

    fn parenthesize_one<E: Expr>(&self, name: &str, expr: &E) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        builder.push_str(&expr.accept::<String>(self));
        builder.push(')');
        builder
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr<L: Expr, R: Expr>(&self, expr: &Binary<L, R>) -> String {
        self.parenthesize_two(&expr.operator.lexeme, &expr.left, &expr.right)
    }
    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> String {
        self.parenthesize_one("group ", &expr.expression)
    }
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> String {
        match expr.value {
            NoneLiteral => String::from("nil"),
            _ => format!("{:?}", expr.value),
        }
    }
    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> String {
        self.parenthesize_one(&expr.operator.lexeme, &expr.right)
    }
}
