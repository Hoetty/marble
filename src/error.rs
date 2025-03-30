use std::fmt::{Debug, Display};

use crate::{source::Source, token::Token, value::ValueRef};

#[derive(Debug, Clone)]
pub struct AnnotatedError {
    pub error: Error,
    pub token: Token,
}

#[derive(Clone)]
pub enum Error {
    ExpectedIdentifierAsVariableName,
    ExpectedIdentifierAsFunctionArgument,
    ExpectedBeInAssignment,
    ExpectedInAfterAssignment,
    ExpectedEofAfterExpression,
    ExpectedExpressionFound(Token),
    ExpectedEndAfterDoBlock,
    ExpectedDoAsFunctionBody,
    ValueNotCallable(ValueRef),
    IdentifierIsNotDefined(String),
    ArgumentToOperatorMustBeANumber(&'static str),
    ArgumentToImportMustBeAString,
    ImportCouldNotBeResolved(String),
    ErrorInImportedFile(String, String),
    ValueDependsOnItself,
    OutputNotWritable,
}

pub enum ErrorType {
    Compile,
    Runtime,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Compile => f.write_str("Compile Error"),
            ErrorType::Runtime => f.write_str("Runtime Error"),
        }
    }
}

impl AnnotatedError {
    pub fn new(error: Error, token: Token) -> AnnotatedError {
        AnnotatedError { error, token }
    }

    pub fn of_source(&self, source: &Source) -> String {
        let line_col = source.start(&self.token);
        format!(
            "{} at {}:{} => '{}'\n{}",
            self.error.error_type(),
            line_col.line + 1,
            line_col.col + 1,
            source.lexeme(&self.token),
            self.error
        )
    }
}

impl Error {
    pub fn annotate(self, token: Token) -> AnnotatedError {
        AnnotatedError::new(self, token)
    }

    pub fn error_type(&self) -> ErrorType {
        match self {
            Error::ExpectedIdentifierAsVariableName
            | Error::ExpectedIdentifierAsFunctionArgument
            | Error::ExpectedBeInAssignment
            | Error::ExpectedInAfterAssignment
            | Error::ExpectedEofAfterExpression
            | Error::ExpectedExpressionFound(_)
            | Error::ExpectedEndAfterDoBlock
            | Error::ExpectedDoAsFunctionBody
            | Error::IdentifierIsNotDefined(_) => ErrorType::Compile,
            _ => ErrorType::Runtime,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedIdentifierAsVariableName => {
                f.write_str("Expected an identifier after 'let'.")
            }
            Self::ExpectedIdentifierAsFunctionArgument => {
                f.write_str("Expected an identifier for a function argument after 'fn'.")
            }
            Self::ExpectedBeInAssignment => f.write_str("Expected 'be' in 'let' assignment."),
            Self::ExpectedInAfterAssignment => f.write_str("Expected 'in' after assignment."),
            Self::ExpectedEofAfterExpression => {
                f.write_str("Expected Eof after expression. Did you miss a function call ('of')?")
            }
            Self::ExpectedExpressionFound(token) => {
                f.write_fmt(format_args!("Expected expression, found {:?}", token))
            }
            Self::ExpectedEndAfterDoBlock => {
                f.write_str("Expected 'end' after do block. Did you miss a function call ('of')?")
            }
            Self::ExpectedDoAsFunctionBody => f.write_str("Expected do to start function body."),
            Self::ValueNotCallable(value) => {
                f.write_fmt(format_args!("{} value is not callable.", value.get_type()))
            }
            Self::IdentifierIsNotDefined(ident) => {
                f.write_fmt(format_args!("Identifier {ident} is not defined."))
            }
            Self::ArgumentToOperatorMustBeANumber(str) => {
                f.write_fmt(format_args!("Argument to {str} must be a number!"))
            }
            Self::ValueDependsOnItself => f.write_str("Calculation of value depends on itself."),
            Self::OutputNotWritable => f.write_str("Outputstream is not writable."),
            Self::ArgumentToImportMustBeAString => {
                f.write_str("Argument to 'Import' must be a string.")
            }
            Self::ImportCouldNotBeResolved(source) => {
                f.write_fmt(format_args!("Import '{source}' could not be resolved."))
            }
            Self::ErrorInImportedFile(file, error) => {
                f.write_fmt(format_args!("Error in imported file '{file}': \n{error}"))
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
