use crate::interpreter::RuntimeError;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralVal {
    Number(f32),
    Str(String),
    Boolean(bool),
    Nil,
    NaN,
    UnInit,
}

impl LiteralVal {
    pub fn number_operand(&self, operator: Token) -> Result<f32, RuntimeError> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::NaN => Ok(f32::NAN),
            _ => Err(RuntimeError(
                Some(operator),
                "Operand must be a number.".to_string(),
            )),
        }
    }
    pub fn print(&self) -> Result<String, RuntimeError> {
        match self {
            Self::Number(n) => {
                let mut res = n.to_string();
                if res.ends_with(".0") {
                    res.pop();
                    res.pop();
                }
                Ok(res)
            }
            Self::Str(s) => Ok(s.clone()),
            Self::Boolean(b) => Ok(b.to_string()),
            Self::Nil => Ok("nil".to_string()),
            Self::NaN => Ok("Nan".to_string()),
            Self::UnInit => Err(RuntimeError(
                None,
                "Can't print unitiliazed variable.".to_string(),
            )),
        }
    }
}

impl Into<Result<LiteralVal, RuntimeError>> for LiteralVal {
    fn into(self) -> Result<LiteralVal, RuntimeError> {
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralVal,
    pub line: u32,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /// One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /// Literals
    Identifier,
    String,
    Number,

    /// Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}
