use crate::{expr::Expr, token::Token};

pub enum Stmt {
    // Block(Vec<Stmt>),
    Expression(Expr),
    // Function {
    //     name: Token,
    //     params: Vec<Token>,
    //     body: Vec<Stmt>,
    // },
    // If {
    //     condition: Expr,
    //     then_branch: Box<Stmt>,
    //     else_branch: Box<Stmt>,
    // },
    Print(Expr),
    // Return(Token, Expr),
    Var(Token, Expr),
    // While(Expr, Box<Stmt>),
}
