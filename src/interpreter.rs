use std::{cell::RefCell, rc::Rc};

type EnvRef<'a> = &'a Rc<RefCell<Environment>>;

use crate::{
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::{LiteralVal, Token, TokenType},
    Lox,
};

#[derive(Debug)]
pub struct RuntimeError(pub Option<Token>, pub String);

use LiteralVal::Nil;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn interpret(&mut self, lox: &mut Lox, statements: Vec<Stmt>, environment: EnvRef) {
        for statement in statements {
            match self.execute(lox, environment, statement) {
                Err(e) => {
                    lox.runtime_error(e);
                    break;
                }
                _ => (),
            }
        }
    }
    pub fn evaluate(
        &mut self,
        lox: &mut Lox,
        environment: EnvRef,
        expr: &Expr,
    ) -> Result<LiteralVal, RuntimeError> {
        let mut res = match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(lox, environment, left.as_ref());
                let right = self.evaluate(lox, environment, right.as_ref());

                match operator.token_type {
                    TokenType::Greater => LiteralVal::Boolean(
                        left?.number_operand(operator.clone())?
                            > right?.number_operand(operator.clone())?,
                    )
                    .into(),
                    TokenType::GreaterEqual => LiteralVal::Boolean(
                        left?.number_operand(operator.clone())?
                            >= right?.number_operand(operator.clone())?,
                    )
                    .into(),
                    TokenType::Less => LiteralVal::Boolean(
                        left?.number_operand(operator.clone())?
                            < right?.number_operand(operator.clone())?,
                    )
                    .into(),
                    TokenType::LessEqual => LiteralVal::Boolean(
                        left?.number_operand(operator.clone())?
                            <= right?.number_operand(operator.clone())?,
                    )
                    .into(),

                    TokenType::BangEqual => {
                        LiteralVal::Boolean(!self.is_equal(&left?, &right?)).into()
                    }

                    TokenType::EqualEqual => {
                        LiteralVal::Boolean(self.is_equal(&left?, &right?)).into()
                    }

                    TokenType::Minus => LiteralVal::Number(
                        left?.number_operand(operator.clone())?
                            - right?.number_operand(operator.clone())?,
                    )
                    .into(),
                    TokenType::Slash => {
                        let right_val = right?.number_operand(operator.clone())?;
                        if right_val == 0. {
                            Err(RuntimeError(
                                Some(operator.clone()),
                                "Division by zero.".to_string(),
                            ))
                        } else {
                            LiteralVal::Number(left?.number_operand(operator.clone())? / right_val)
                                .into()
                        }
                    }
                    TokenType::Star => LiteralVal::Number(
                        left?.number_operand(operator.clone())?
                            * right?.number_operand(operator.clone())?,
                    )
                    .into(),
                    TokenType::Plus => match (left, right) {
                        (Ok(LiteralVal::Number(nl)), Ok(LiteralVal::Number(nr))) => {
                            Ok(LiteralVal::Number(nl + nr))
                        }
                        (Ok(LiteralVal::Str(sl)), Ok(LiteralVal::Str(sr))) => {
                            Ok(LiteralVal::Str(sl + &sr))
                        }
                        _ => Err(RuntimeError(
                            Some(operator.clone()),
                            "Operands must be two numbers or two strings.".to_string(),
                        )),
                    },
                    _ => unreachable!(),
                }
            }

            Expr::Grouping(expr) => self.evaluate(lox, environment, expr.as_ref()),

            Expr::Literal(value) => Ok(value.clone()),

            Expr::Unary { operator, right } => {
                let right = self.evaluate(lox, environment, right.as_ref());

                match operator.token_type {
                    TokenType::Minus => {
                        LiteralVal::Number(right?.number_operand(operator.clone())? * -1.).into()
                    }

                    TokenType::Bang => Ok(LiteralVal::Boolean(!self.is_truthy(&right?))),
                    _ => unreachable!(),
                }
            }

            Expr::Variable(name) => environment.borrow().get(name),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(lox, environment, expr.as_ref())?;
                environment.borrow_mut().assign(name, value.clone())?;
                Ok(value)
            }
        };
        match res {
            Ok(LiteralVal::Number(n)) => {
                if n.is_nan() {
                    res = Ok(LiteralVal::NaN)
                }
            }
            _ => (),
        }
        res
    }

    fn execute(
        &mut self,
        lox: &mut Lox,
        environment: EnvRef,
        stmt: Stmt,
    ) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expression) => match self.evaluate(lox, environment, &expression) {
                Err(e) => Err(e),
                _ => Ok(()),
            },

            Stmt::Print(expression) => {
                let value = self.evaluate(lox, environment, &expression);
                match value {
                    Ok(val) => {
                        println!("{}", val.print()?);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }

            Stmt::Var(name, init) => {
                // To revert Chapter8/Challenge2, comment the line below and uncomment the one below that.
                let mut value = LiteralVal::UnInit;
                // let mut value = LiteralVal::Nil;
                if let Some(expr) = init {
                    value = self.evaluate(lox, environment, &expr)?;
                }

                environment.borrow_mut().define(name.lexeme, value);
                Ok(())
            }

            Stmt::Block(statements) => self.execute_block(
                lox,
                &statements,
                Environment::with_enclosing(Rc::clone(environment)),
            ),
        }
    }

    fn execute_block(
        &mut self,
        lox: &mut Lox,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let er = Rc::new(RefCell::new(environment));

        for stmt in statements {
            self.execute(lox, &er, stmt.clone())?;
        }
        Ok(())
    }

    pub fn is_truthy(&self, obj: &LiteralVal) -> bool {
        match obj {
            Nil | LiteralVal::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn is_equal(&self, a: &LiteralVal, b: &LiteralVal) -> bool {
        a == b
    }
}
