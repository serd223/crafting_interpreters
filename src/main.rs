use clap::Parser;
use crafting_interpreters::{interpreter::Interpreter, Lox};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let Args { file } = Args::parse();
    let mut lox = Lox::default();
    let mut interpreter = Interpreter {};
    match file {
        Some(f) => lox.run_file(f, &mut interpreter),
        None => lox.run_prompt(&mut interpreter),
    }
}
