use std::{io::Write, path::Path};

pub mod scanner;
use scanner::Scanner;
pub mod token;
pub struct Lox {
    had_error: bool,
}

impl Default for Lox {
    fn default() -> Self {
        Self { had_error: false }
    }
}

impl Lox {
    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        for token in tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: u32, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    pub fn run_file<T: AsRef<Path>>(&mut self, file: T) {
        let source = std::fs::read_to_string(file).unwrap();
        self.run(source);
        if self.had_error {
            panic!("had error")
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => (),
                Err(_) => panic!("couldn't read from stdin"),
            }
            self.run(line);
        }
    }
}
