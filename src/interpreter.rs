use crate::{
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::{LiteralVal, Token, TokenType},
    Lox,
};

pub struct RuntimeError(pub Token, pub String);

use LiteralVal::Nil;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, lox: &mut Lox, statements: Vec<Stmt>) {
        for statement in statements {
            match self.execute(lox, statement) {
                Err(e) => {
                    lox.runtime_error(e);
                    break;
                }
                _ => (),
            }
        }
    }
    fn evaluate(&mut self, lox: &mut Lox, expr: &Expr) -> Result<LiteralVal, RuntimeError> {
        let mut res = match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(lox, left.as_ref());
                let right = self.evaluate(lox, right.as_ref());

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
                                operator.clone(),
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
                            operator.clone(),
                            "Operands must be two numbers or two strings.".to_string(),
                        )),
                    },
                    _ => unreachable!(),
                }
            }

            Expr::Grouping(expr) => self.evaluate(lox, expr.as_ref()),

            Expr::Literal(value) => Ok(value.clone()),

            Expr::Unary { operator, right } => {
                let right = self.evaluate(lox, right.as_ref());

                match operator.token_type {
                    TokenType::Minus => {
                        LiteralVal::Number(right?.number_operand(operator.clone())? * -1.).into()
                    }

                    TokenType::Bang => Ok(LiteralVal::Boolean(!self.is_truthy(&right?))),
                    _ => unreachable!(),
                }
            }

            Expr::Variable(name) => self.environment.get(name),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(lox, expr.as_ref())?;
                self.environment.assign(name, value.clone())?;
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

    fn execute(&mut self, lox: &mut Lox, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expression) => match self.evaluate(lox, &expression) {
                Err(e) => Err(e),
                _ => Ok(()),
            },

            Stmt::Print(expression) => {
                let value = self.evaluate(lox, &expression);
                match value {
                    Ok(val) => {
                        println!("{}", val.to_string());
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }

            Stmt::Var(name, init) => {
                let mut value = LiteralVal::Nil;
                if let Some(expr) = init {
                    value = self.evaluate(lox, &expr)?;
                }

                self.environment.define(name.lexeme, value);
                Ok(())
            }
        }
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
