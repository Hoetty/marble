use std::path::PathBuf;

mod src;

use clap::Parser;
use marble::{evaluate_file, evaluate_string};
use src::repl::input;

/// Marble interpreter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional file to execute
    #[arg()]
    file: Option<PathBuf>,
}

pub fn main() {
    let args = Args::parse();

    if let Some(path) = args.file {
        run_file(&path);
    } else {
        repl();
    }
}

fn repl() {
    for line in input() {
        match evaluate_string(&line) {
            Ok(value) => println!("{value}"),
            Err(e) => println!("Error -> {e}"),
        }
    }
}

fn run_file(file: &PathBuf) {
    match evaluate_file(file) {
        Ok(value) => println!("{value}"),
        Err(e) => println!("Error -> {e}"),
    }
}