use std::iter::Peekable;

use crate::{builtin, error::Error, expr::{Expr, ExprRef}, scanner::Scanner, source::{IdentifierTable, Source}, token::{Token, TokenType}, value::{Value, ValueRef}};

type ExprResult = Result<ExprRef, Error>;

type Binding<'a> = (&'a str, fn () -> ExprRef);

pub struct Compiler<'a> {
    source: &'a Source<'a>,
    scanner: Peekable<Scanner<'a>>,
    identifiers: IdentifierTable<'a>,
    extra_bindings: Vec<Binding<'a>>
}

impl <'a> Compiler<'a> {

    pub fn default_bindings() -> Vec<Binding<'static>> {
        vec![
            ("True", builtin::get_true),
            ("False", builtin::get_false),
            ("And", builtin::get_and),
            ("Or", builtin::get_or),
            ("Not", builtin::get_not),
            ("If", builtin::get_if),
            ("Unit", || ExprRef::new(Expr::Value(ValueRef::new(Value::Unit)))),
            ("PrintLn", builtin::get_println),
            ("Print", builtin::get_print),
            ("Is", builtin::get_is),
            ("IsNot", builtin::get_is_not),
            ("Add", builtin::get_add),
            ("Sub", builtin::get_sub),
            ("Mul", builtin::get_mul),
            ("Div", builtin::get_div),
        ]
    }

    pub fn new(source: &'a Source<'a>, scanner: Scanner<'a>) -> Compiler<'a> {
        Self {
            source,
            scanner: scanner.peekable(),
            identifiers: IdentifierTable::new(),
            extra_bindings: Vec::new()
        }
    }

    pub fn with_bindings(&mut self, bindings: Vec<Binding<'a>>) -> &Self {
        self.extra_bindings.extend(bindings);

        self
    }

    pub fn compile(mut self) -> Result<(ExprRef, IdentifierTable<'a>), (Token, Error)> {
        for (ident, _) in &self.extra_bindings {
            self.identifiers.push(ident).map_err(|e| (Token { end: 0, start: 0, token_type: TokenType::Eof }, e))?;
        }

        let mut expr = self.expression().map_err(|e| (self.consume(), e))?;

        for (_, provider) in self.extra_bindings.iter().rev() {
            let function = ExprRef::new(Expr::Fn(expr));
            expr = ExprRef::new(Expr::Call(function, provider()))
        }

        self.match_consume(TokenType::Eof, Error::ExpectedEofAfterExpression).map_err(|e| (self.consume(), e))?;

        Ok((expr, self.identifiers))
    }

    fn expression(&mut self) -> ExprResult {
        self.then_expression()
    }

    fn let_expression(&mut self) -> ExprResult {
        if self.matches(TokenType::Let).is_none() {
            return self.call();
        }

        let variable_name = self.try_identifier()?;

        self.match_consume(TokenType::Be, Error::ExpectedBeInAssignment)?;

        let value = self.then_expression()?;

        self.match_consume(TokenType::In, Error::ExpectedInAfterAssignment)?;

        // After the initiliazer is finished, the identifier is pushed, so that it isnt available in the initializer
        self.identifiers.push(variable_name)?;

        let body = self.then_expression()?;

        self.identifiers.pop();

        let function = ExprRef::new(Expr::Fn(body));
        
        Ok(ExprRef::new(Expr::Call(function, value)))
    }

    fn then_expression(&mut self) -> ExprResult {
        let mut lhs = self.let_expression()?;

        while self.matches(TokenType::Then).is_some() {
            let rhs = self.let_expression()?;
            lhs = ExprRef::new(Expr::Then(lhs, rhs));
        }

        Ok(lhs)
    }

    fn call(&mut self) -> ExprResult {
        let mut lhs = self.value()?;

        while self.matches(TokenType::Of).is_some() {
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
                self.identifiers.distance_from_top(self.source.lexeme(&token)).map(|ident| {
                    ExprRef::new(Expr::Identifier(ident))
                })
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
        let mut arguments = vec![self.try_identifier()?];

        while let Some(token) = self.matches(TokenType::Identifier) {
            arguments.push(&self.source.lexeme(&token));
        }

        for identifier in &arguments {
            self.identifiers.push(identifier)?;
        }

        self.match_consume(TokenType::Do, Error::ExpectedDoAsFunctionBody)?;

        let body = self.block()?;

        let mut expr = body;

        for _ in 0..arguments.len() {
            expr = ExprRef::new(Expr::Fn(expr));
            self.identifiers.pop();
        }

        Ok(expr)
    }

    fn try_identifier(&mut self) -> Result<&'a str, Error> {
        let identifier = self.match_consume(TokenType::Identifier, Error::ExpectedIdentifier)?;

        Ok(self.source.lexeme(&identifier))
    }

    fn peek(&mut self) -> &Token {
        self.scanner.peek().unwrap()
    }

    fn consume(&mut self) -> Token {
        self.scanner.next().unwrap()
    }

    fn matches(&mut self, token: TokenType) -> Option<Token> {
        if self.peek().token_type == token {
            Some(self.consume())    
        } else {
            None
        }
    }

    fn match_consume(&mut self, token: TokenType, error: Error) -> Result<Token, Error> {
        self.matches(token).ok_or(error)
    }
}