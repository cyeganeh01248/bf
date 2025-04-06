mod interpreter;

use clap::Parser;
use interpreter::Interpreter;

use std::fs;

#[derive(Debug, Parser)]
struct Args {
    program: String,
}

fn main() {
    let args = Args::parse();
    let program = fs::read_to_string(args.program).unwrap();

    let mut interpreter = Interpreter::new(&program).unwrap();
    let _ = interpreter.run();
}
