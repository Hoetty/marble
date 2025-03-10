use std::fmt::{Debug, Display};

use crate::{token::Token, value::ValueRef};

#[derive(Clone)]
pub enum Error {
    ExpectedIdentifier,
    ExpectedBeInAssignment,
    ExpectedInAfterAssignment,
    ExpectedEofAfterExpression,
    ExpectedExpressionFound(Token),
    ExpectedEndAfterDoBlock,
    ExpectedDoAsFunctionBody,
    ValueNotCallable(ValueRef),
    IdentifierIsNotDefined(String),
    IdentifierIsAlreadyDefined(String),
    ArgumentToOperatorMustBeANumber(&'static str),
    ValueDependsOnItself,
    OutputNotWritable,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedIdentifier => f.write_str("Expected identifier."),
            Self::ExpectedBeInAssignment => f.write_str("Expected 'be' in assignment."),
            Self::ExpectedInAfterAssignment => f.write_str("Expected 'in' after assignment."),
            Self::ExpectedEofAfterExpression => f.write_str("Expected Eof after expression."),
            Self::ExpectedExpressionFound(token) => f.write_fmt(format_args!("Expected expression, found {:?}", token)),
            Self::ExpectedEndAfterDoBlock => f.write_str("Expected 'end' after do block"),
            Self::ExpectedDoAsFunctionBody => f.write_str("Expected do to start function body"),
            Self::ValueNotCallable(value) => f.write_fmt(format_args!("{} value is not callable", value.get_type())),
            Self::IdentifierIsNotDefined(ident) => f.write_fmt(format_args!("Identifier {ident} is not defined")),
            Self::IdentifierIsAlreadyDefined(ident) => f.write_fmt(format_args!("Identifier {ident} is already defined")),
            Self::ArgumentToOperatorMustBeANumber(str) => f.write_fmt(format_args!("Argument to {str} must be a number!")),
            Self::ValueDependsOnItself => f.write_str("Calculation on value depends on itself"),
            Self::OutputNotWritable => f.write_str("Output is not writable"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}