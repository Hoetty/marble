use std::{fs::read_to_string, path::PathBuf, process::exit};

mod src;

use clap::Parser;
use marble::{compiler::Compiler, interpreter::Interpreter, number::serialize, scanner::Scanner, source::Source, token::TokenType};
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
        // let source = Source::new(&line);
        // let mut scanner = Scanner::new(source);
    
        // run_scanner(&mut scanner, &source);

        if let Ok(num) = line.parse() {
            println!("{}", serialize::display_number(num));
        }

        // println!("{:?}", deserialize::parse_fraction(&line));
    }
}

pub fn file(file: &PathBuf) {
    let file = read_to_string(file).unwrap();
    let source = Source::new(&file);
    let scanner = Scanner::new(source);

    let compiler = Compiler::new(&source, scanner);
    let result = compiler.compile();

    match result {
        Ok((expr, table)) => {
            let mut interpreter = Interpreter::new(expr, &source, table);

            println!("{}", interpreter.interpret())
        },
        Err((token, error)) => {
            let line = source.line_start(&token);
            let column = source.column_start(&token);
            let lexeme = source.lexeme(&token);

            println!("Error at '{lexeme}' {line}:{column} -> {error}");
            exit(1);
        },
    }
}

// fn run_scanner(scanner: &mut Scanner, source: &Source) {
//     loop {
//         let token = scanner.next().unwrap();
//         println!("{:?}: {:?}", token, source.lexeme(&token));
    
//         if token.token_type == TokenType::Eof {
//             break;
//         }
//     }
// }