use std::{
    fs::read_to_string,
    io::{Cursor, Read, Write, stdin, stdout},
    path::PathBuf,
    str::FromStr,
};

use compiler::Compiler;
use error::AnnotatedError;
use interpreter::{Interpreter, ValueResult};
use scanner::Scanner;
use source::Source;
use value::ValueRef;

pub mod builtin;
pub mod compiler;
pub mod environment;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod meta;
pub mod scanner;
pub mod source;
pub mod token;
pub mod value;

#[cfg(test)]
pub mod tests;

/// A crate to handle numbers and their written forms
///  
/// Provides utility to convert numbers to words
/// ```rust
/// use marble::number::serialize;
/// assert_eq!(serialize::display_number(123), "OneHundredTwentyThree");
/// ```
///
/// And utility to parse such strings to numbers
///
/// ```rust
/// use marble::number::deserialize;
/// assert_eq!(deserialize::parse_number("OneHundredTwentyThree"), Some(123));
/// ```
///
/// Please note, that serialization will always return a string directly,
/// because all numbers can be converted to a word. However in contrast, desirialization
/// returns an Option, as it may fail. *```"Banana"``` isn't a number after all*.
pub mod number;

pub type OutputResult = Result<(ValueRef, String), AnnotatedError>;

pub fn evaluate_file(file: &PathBuf) -> ValueResult {
    let file = read_to_string(file).unwrap();
    evaluate_string(&file)
}

pub fn evaluate_file_at(file: &str) -> ValueResult {
    evaluate_file(&PathBuf::from_str(file).unwrap())
}

pub fn evaluate_string(code: &str) -> ValueResult {
    evaluate_code(code, stdin(), stdout())
}

pub fn execute_string(code: &str) -> OutputResult {
    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    evaluate_code(code, stdin(), cursor).map(move |val| (val, String::from_utf8(output).unwrap()))
}

pub fn execute_file_at(file: &str) -> OutputResult {
    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    let code = read_to_string(file).unwrap();
    evaluate_code(&code, stdin(), cursor).map(move |val| (val, String::from_utf8(output).unwrap()))
}

pub fn evaluate_code<I: Read, O: Write>(code: &str, input: I, output: O) -> ValueResult {
    let source = Source::new(code);
    let scanner = Scanner::new(&source);

    let mut compiler = Compiler::new(&source, scanner);
    compiler.with_bindings(Compiler::default_bindings());
    let expr = compiler.compile()?;

    let mut interpreter = Interpreter::new(expr, input, output);
    interpreter.interpret()
}
