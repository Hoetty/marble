use std::{
    fs::read_to_string,
    io::{Cursor, stdin, stdout},
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use compiler::Compiler;
use error::AnnotatedError;
use interpreter::{Input, Interpreter, Output, ValueResult};
use scanner::Scanner;
use source::Source;
use value::ValueRef;

pub mod builtin;
pub mod compiler;
pub mod environment;
pub mod error;
pub mod expr;
pub mod identifier;
pub mod interpreter;
pub mod meta;
pub mod scanner;
pub mod source;
pub mod token;
pub mod value;

#[cfg(test)]
pub mod tests;

pub mod number;

pub type OutputResult = Result<(ValueRef, String), AnnotatedError>;

pub fn evaluate_file(file: &PathBuf) -> ValueResult {
    let code = read_to_string(file).unwrap();
    evaluate_string(&code, file.parent().unwrap().to_path_buf())
}

pub fn evaluate_file_at(file: &str) -> ValueResult {
    evaluate_file(&PathBuf::from_str(file).unwrap())
}

pub fn evaluate_string(code: &str, execution_path: PathBuf) -> ValueResult {
    evaluate_code(
        code,
        Arc::new(Mutex::new(Box::new(stdin()))),
        Arc::new(Mutex::new(Box::new(stdout()))),
        execution_path,
    )
}

pub fn execute_string(code: &str, execution_path: PathBuf) -> OutputResult {
    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    evaluate_code(
        code,
        Arc::new(Mutex::new(Box::new(stdin()))),
        Arc::new(Mutex::new(Box::new(cursor))),
        execution_path,
    )
    .map(move |val| (val, String::from_utf8(output).unwrap()))
}

pub fn execute_file_at(file: &str) -> OutputResult {
    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    let code = read_to_string(file).unwrap();
    evaluate_code(
        &code,
        Arc::new(Mutex::new(Box::new(stdin()))),
        Arc::new(Mutex::new(Box::new(cursor))),
        PathBuf::from(file).parent().unwrap().to_path_buf(),
    )
    .map(move |val| (val, String::from_utf8(output).unwrap()))
}

pub fn evaluate_code<'a>(
    code: &str,
    input: Input<'a>,
    output: Output<'a>,
    execution_path: PathBuf,
) -> ValueResult {
    let source = Source::new(code);
    let scanner = Scanner::new(&source);

    let mut compiler = Compiler::new(&source, scanner);
    compiler.with_bindings(Compiler::default_bindings());
    let expr = compiler.compile()?;

    let mut interpreter = Interpreter::new(input, output, execution_path);
    interpreter.interpret(expr)
}
