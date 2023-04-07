use crate::{
    expr::Expr,
    token::{LiteralVal, Token, TokenType},
    Lox,
};

use LiteralVal::Nil;

pub struct ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self, lox: &mut Lox) -> Option<Expr> {
        self.expression(lox).ok()
    }

    fn expression(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        self.equality(lox)
    }

    fn equality(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        let mut expr = self.comparison(lox)?;

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison(lox)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        let mut expr = self.term(lox)?;

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term(lox)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        let mut expr = self.factor(lox)?;

        while self.match_types(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor(lox)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        let mut expr = self.unary(lox)?;

        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary(lox)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary(lox)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary(lox)
    }

    fn primary(&mut self, lox: &mut Lox) -> Result<Expr, ParserError> {
        if self.match_types(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralVal::Boolean(false)));
        }
        if self.match_types(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralVal::Boolean(true)));
        }
        if self.match_types(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Nil));
        }

        if self.match_types(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.expression(lox)?;
            match self.consume(lox, &TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_) => return Ok(Expr::Grouping(Box::new(expr))),
                Err(_) => return Err(ParserError),
            }
        }

        self.error(lox, self.peek(), "Expect expression");
        Err(ParserError)
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(
        &mut self,
        lox: &mut Lox,
        t_type: &TokenType,
        message: &str,
    ) -> Result<&Token, ParserError> {
        if self.check(t_type) {
            return Ok(self.advance());
        }

        self.error(lox, self.peek(), message);
        Err(ParserError)
    }

    fn check(&self, t_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == t_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, lox: &mut Lox, token: &Token, message: &str) {
        lox.error_token(token, message);
    }
}
