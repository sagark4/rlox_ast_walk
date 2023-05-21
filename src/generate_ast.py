contents = []
type_desc = [
      "Binary   : Expr left, Token operator, Expr right",
      "Grouping : Expr expression",
      "LiteralExpr  : Literal value",
      "Unary    : Token operator, Expr right"
]
contents.append("use crate::token::Token;")
contents.append("use crate::token::Literal;")
contents.append("")
contents.append("pub(crate) trait Expr {")
contents.append("    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;")
contents.append("}")
contents.append("")
contents.append("pub(crate) trait Visitor<R> {")
for td in type_desc:
    typ = td.split(":")[0].strip()
    if "Expr" in td.split():
        contents.append("    fn visit_"+typ.lower()+"_expr<E: Expr>(&self, expr: &"+typ+"<E>) -> R;")
    else:
        contents.append("    fn visit_"+typ.lower()+"_expr(&self, expr: &"+typ+") -> R;")
contents.append("}")
contents.append("")

for td in type_desc:
    typ = td.split(":")[0].strip()
    fields = td.split(":")[1].strip().split(", ")
    if "Expr" in td.split():
        contents.append("pub(crate) struct "+typ+"<E: Expr> {")
        for field in fields:
            ftype = field.split()[0]
            fname = field.split()[1]
            if ftype == 'Expr':
                contents.append("    "+fname+": E,")
            else:
                contents.append("    "+fname+": "+ftype+",")
        contents.append("}")
        contents.append("")
        contents.append("impl<E: Expr> Expr for "+typ+"<E> {")
        contents.append("    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {")
        contents.append("        visitor.visit_"+typ.lower()+"_expr(&self)")
        contents.append("    }")
        contents.append("}")
        contents.append("")
    else:
        contents.append("pub(crate) struct "+typ+" {")
        for field in fields:
            ftype = field.split()[0]
            fname = field.split()[1]
            contents.append("    "+fname+": "+ftype+",")
        contents.append("}")
        contents.append("")
        contents.append("impl Expr for "+typ+" {")
        contents.append("    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {")
        contents.append("        visitor.visit_"+typ.lower()+"_expr(&self)")
        contents.append("    }")
        contents.append("}")
        contents.append("")
with open('expr.rs', 'w', encoding="utf-8") as f:
    f.write("\n".join(contents))
    