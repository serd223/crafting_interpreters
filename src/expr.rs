use crate::token::{LiteralVal, Token};

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralVal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

fn paranthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut res = String::new();
    res.push('(');
    res.push_str(name);
    for expr in exprs {
        res.push(' ');
        res.push_str(&expr.to_string());
    }
    res.push(')');

    res
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => paranthesize(&operator.lexeme, &[left.as_ref(), right.as_ref()]),
            Self::Grouping(expr) => paranthesize("group", &[expr.as_ref()]),
            Self::Literal(val) => val.to_string(),
            Self::Unary { operator, right } => paranthesize(&operator.lexeme, &[right.as_ref()]),
        }
    }
}
