mod scanner;
mod token;

use clap::Parser;
use scanner::Scanner;
use std::{io::Write, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

struct Lox {
    had_error: bool,
}

impl Default for Lox {
    fn default() -> Self {
        Self { had_error: false }
    }
}

impl Lox {
    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, String::new(), message);
    }

    fn report(&mut self, line: u32, where_: String, message: String) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    fn run_file<T: AsRef<Path>>(&mut self, file: T) {
        let source = std::fs::read_to_string(file).unwrap();
        self.run(source);
        if self.had_error {
            panic!("had error")
        }
    }

    fn run_prompt(&mut self) {
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
fn main() {
    let Args { file } = Args::parse();
    let mut lox = Lox::default();
    match file {
        Some(f) => lox.run_file(f),
        None => lox.run_prompt(),
    }
}
