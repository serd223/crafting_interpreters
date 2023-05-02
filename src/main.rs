use std::{cell::RefCell, rc::Rc};

use clap::Parser;
use crafting_interpreters::{environment::Environment, interpreter::Interpreter, Lox};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let Args { file } = Args::parse();
    let mut lox = Lox::default();
    let mut interpreter = Interpreter::new();
    let environment = Environment::new();
    let env_ref = Rc::new(RefCell::new(environment));
    match file {
        Some(f) => lox.run_file(f, &mut interpreter, &env_ref),
        None => lox.run_prompt(&mut interpreter, &env_ref),
    }
}
