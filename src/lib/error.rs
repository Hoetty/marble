use std::fmt::Display;

use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Error {
    ExpectedIdentifier,
    ExpectedBeInAssignment,
    ExpectedInAfterAssignment,
    ExpectedEofAfterExpression,
    ExpectedExpressionFound(Token),
    ExpectedEndAfterDoBlock,
    ExpectedDoAsFunctionBody
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
        }
    }
}