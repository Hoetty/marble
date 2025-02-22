use std::iter::Peekable;

use crate::{error::Error, expr::{Expr, ExprRef, IdentRef}, scanner::Scanner, source::{IdentifierTable, Source}, token::{Token, TokenType}};

type ExprResult = Result<ExprRef, Error>;
type IdentResult = Result<IdentRef, Error>;

pub struct Compiler<'a> {
    source: &'a Source<'a>,
    scanner: Peekable<Scanner<'a>>,
    identifiers: IdentifierTable<'a>
}

impl <'a> Compiler<'a> {

    pub fn new(source: &'a Source<'a>, scanner: Scanner<'a>) -> Compiler<'a> {
        Self {
            source,
            scanner: scanner.peekable(),
            identifiers: IdentifierTable::new()
        }
    }

    pub fn compile(mut self) -> Result<(ExprRef, IdentifierTable<'a>), (Token, Error)> {
        let expr = self.expression().map_err(|e| (self.consume(), e))?;

        self.match_consume(TokenType::Eof, Error::ExpectedEofAfterExpression).map_err(|e| (self.consume(), e))?;

        Ok((expr, self.identifiers))
    }

    fn expression(&mut self) -> ExprResult {
        self.then_expression()
    }

    fn let_expression(&mut self) -> ExprResult {
        if !self.matches(TokenType::Let) {
            return self.call();
        }

        let ident = self.try_identifier()?;
        self.match_consume(TokenType::Be, Error::ExpectedBeInAssignment)?;

        let value = self.then_expression()?;

        self.match_consume(TokenType::In, Error::ExpectedInAfterAssignment)?;

        let body = self.then_expression()?;

        let function = ExprRef::new(Expr::Fn(ident, body));
        
        Ok(ExprRef::new(Expr::Call(function, value)))
    }

    fn then_expression(&mut self) -> ExprResult {
        let mut lhs = self.let_expression()?;

        while self.matches(TokenType::Then) {
            let rhs = self.let_expression()?;
            lhs = ExprRef::new(Expr::Then(lhs, rhs));
        }

        Ok(lhs)
    }

    fn call(&mut self) -> ExprResult {
        let mut lhs = self.value()?;

        while self.matches(TokenType::Of) {
            let rhs = self.value()?;
            lhs = ExprRef::new(Expr::Call(lhs, rhs));
        }

        Ok(lhs)
    }

    fn value(&mut self) -> ExprResult {
        let token = self.consume();
        match token.token_type {
            TokenType::Do => self.block(),
            TokenType::Fn => self.function(),
            TokenType::String => {
                let lexeme = self.source.lexeme(&token);
                match lexeme {
                    "string" => Ok(self.string_of("")),
                    _ if lexeme.len() == 7 => Ok(self.string_of("")),
                    _ => Ok(self.string_of(&lexeme[4..lexeme.len() - 4])),
                }
            },
            TokenType::Number(num) => Ok(ExprRef::new(Expr::Number(num))),
            TokenType::Identifier => {
                let ident = self.identifiers.reference(&self.source.lexeme(&token));
                Ok(ExprRef::new(Expr::Identifier(ident)))
            },
            _ => Err(Error::ExpectedExpressionFound(token))
        }
    }

    fn string_of(&mut self, string: &str) -> ExprRef {
        ExprRef::new(Expr::String(string.to_string()))
    }

    fn block(&mut self) -> ExprResult {
        let expr = self.expression()?;
        self.match_consume(TokenType::End, Error::ExpectedEndAfterDoBlock)?;
        Ok(expr)
    }

    fn function(&mut self) -> ExprResult {
        let ident = self.try_identifier()?;

        self.match_consume(TokenType::Do, Error::ExpectedDoAsFunctionBody)?;

        let body = self.block()?;

        Ok(ExprRef::new(Expr::Fn(ident, body)))
    }

    fn try_identifier(&mut self) -> IdentResult {
        if self.peek().token_type == TokenType::Identifier {
            let identifier = self.consume();
            Ok(self.identifiers.reference(&self.source.lexeme(&identifier)))
        } else {
            Err(Error::ExpectedIdentifier)
        }
    }

    fn peek(&mut self) -> &Token {
        self.scanner.peek().unwrap()
    }

    fn consume(&mut self) -> Token {
        self.scanner.next().unwrap()
    }

    fn matches(&mut self, token: TokenType) -> bool {
        if self.peek().token_type == token {
            self.consume();
            return true;
        }

        false
    }

    fn match_consume(&mut self, token: TokenType, error: Error) -> Result<(), Error> {
        if self.matches(token) {
            Ok(())
        } else {
            Err(error)
        }
    }
}