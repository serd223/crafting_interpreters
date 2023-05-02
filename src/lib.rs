use std::{cell::RefCell, io::Write, path::Path, rc::Rc};

pub mod interpreter;
pub mod parser;
pub mod scanner;
use environment::Environment;
use interpreter::{Interpreter, RuntimeError};
use parser::Parser;
use scanner::Scanner;
use stmt::Stmt;
use token::{Token, TokenType};
pub mod environment;
pub mod expr;
pub mod stmt;
pub mod token;

type EnvRef<'a> = &'a Rc<RefCell<Environment>>;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Default for Lox {
    fn default() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }
}

impl Lox {
    pub fn run(
        &mut self,
        source: String,
        interpreter: &mut Interpreter,
        environment: EnvRef,
        repl: bool,
    ) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);
        // for token in tokens.clone() {
        //     println!("{}", token.to_string());
        // }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse(self);

        if self.had_error {
            return;
        }

        let should_interpret = if repl && statements.len() == 1 {
            match &statements[0] {
                Stmt::Expression(expr) => match interpreter.evaluate(self, environment, expr) {
                    Ok(val) => {
                        match val.print() {
                            Ok(s) => println!("{s}"),
                            Err(e) => self.runtime_error(e),
                        }
                        false
                    }
                    Err(e) => {
                        self.runtime_error(e);
                        false
                    }
                },
                _ => true,
            }
        } else {
            true
        };

        if should_interpret {
            interpreter.interpret(self, statements, environment);
        }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    pub fn runtime_error(&mut self, err: RuntimeError) {
        match err.0 {
            Some(op) => eprintln!("{}\n[line {};token {}]", err.1, op.line, op.lexeme),
            None => eprintln!("{}", err.1),
        };
        self.had_runtime_error = true;
    }

    pub fn report(&mut self, line: u32, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    pub fn error_token(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message)
        } else {
            self.report(
                token.line,
                format!(" at '{}'", token.lexeme).as_str(),
                message,
            )
        }
    }

    pub fn run_file<T: AsRef<Path>>(
        &mut self,
        file: T,
        interpreter: &mut Interpreter,
        environment: EnvRef,
    ) {
        let source = std::fs::read_to_string(file).unwrap();
        self.run(source, interpreter, environment, false);
        if self.had_error {
            panic!("had error")
        }
    }

    pub fn run_prompt(&mut self, interpreter: &mut Interpreter, environment: EnvRef) {
        loop {
            self.had_error = false;
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => (),
                Err(_) => panic!("couldn't read from stdin"),
            }
            self.run(line, interpreter, environment, true);
        }
    }
}
