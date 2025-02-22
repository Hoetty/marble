use std::iter::Peekable;

use crate::{ast::{Ast, Expr}, error::Error, scanner::Scanner, source::{IdentifierTable, Source}, token::{Token, TokenType}};

type ExprResult = Result<usize, Error>;
type IdentResult = Result<usize, Error>;

pub struct Compiler<'a> {
    source: &'a Source<'a>,
    scanner: Peekable<Scanner<'a>>,
    ast: Ast,
    identifiers: IdentifierTable<'a>
}

impl <'a> Compiler<'a> {

    pub fn new(source: &'a Source<'a>, scanner: Scanner<'a>) -> Compiler<'a> {
        Self {
            source,
            scanner: scanner.peekable(),
            ast: Ast::new(),
            identifiers: IdentifierTable::new()
        }
    }

    pub fn compile(mut self) -> Result<(Ast, IdentifierTable<'a>), (Token, Error)> {
        self.expression().map_err(|e| (self.consume(), e))?;

        self.match_consume(TokenType::Eof, Error::ExpectedEofAfterExpression).map_err(|e| (self.consume(), e))?;

        Ok((self.ast, self.identifiers))
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

        let function = self.ast.push_expr(Expr::Fn(ident, body));
        
        Ok(self.ast.push_expr(Expr::Call(function, value)))
    }

    fn then_expression(&mut self) -> ExprResult {
        let mut lhs = self.let_expression()?;

        while self.matches(TokenType::Then) {
            let rhs = self.let_expression()?;
            lhs = self.ast.push_expr(Expr::Then(lhs, rhs));
        }

        Ok(lhs)
    }

    fn call(&mut self) -> ExprResult {
        let mut lhs = self.value()?;

        while self.matches(TokenType::Of) {
            let rhs = self.value()?;
            lhs = self.ast.push_expr(Expr::Call(lhs, rhs));
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
            TokenType::Number(num) => Ok(self.ast.push_expr(Expr::Number(num))),
            TokenType::Identifier => {
                let ident = self.identifiers.reference(&self.source.lexeme(&token));
                Ok(self.ast.push_expr(Expr::Identifier(ident)))
            },
            _ => Err(Error::ExpectedExpressionFound(token))
        }
    }

    fn string_of(&mut self, string: &str) -> usize {
        self.ast.push_expr(Expr::String(string.to_string()))
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

        Ok(self.ast.push_expr(Expr::Fn(ident, body)))
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