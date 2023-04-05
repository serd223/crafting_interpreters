pub mod scanner;
pub mod token;

use clap::Parser;
use crafting_interpreters::Lox;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let Args { file } = Args::parse();
    let mut lox = Lox::default();
    match file {
        Some(f) => lox.run_file(f),
        None => lox.run_prompt(),
    }
}
