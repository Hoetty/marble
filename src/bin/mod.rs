use std::{fs::read_to_string, path::PathBuf};

mod src;

use clap::Parser;
use marble::{scanner::Scanner, source::Source, token::TokenType};
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
        file(&path);
    } else {
        repl();
    }
}

pub fn repl() {
    for line in input() {
        let source = Source::new(&line);
        let mut scanner = Scanner::new(source);
    
        run_scanner(&mut scanner, &source);
    }
}

pub fn file(file: &PathBuf) {
    let file = read_to_string(file).unwrap();
    let source = Source::new(&file);
    let mut scanner = Scanner::new(source);

    run_scanner(&mut scanner, &source);
}

fn run_scanner(scanner: &mut Scanner, source: &Source) {
    loop {
        let token = scanner.next_token();
        println!("{:?}: {:?}", token, source.lexeme(&token));
    
        if token.token_type == TokenType::Eof {
            break;
        }
    }
}