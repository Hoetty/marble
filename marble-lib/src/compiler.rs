use std::iter::Peekable;

use crate::{builtin, error::{AnnotatedError, Error}, expr::{Expr, ExprRef}, scanner::Scanner, source::{IdentifierTable, Source}, token::{Token, TokenType}, value::{Value, ValueRef}};

type ExprResult = Result<ExprRef, AnnotatedError>;

type Binding<'a> = (&'a str, ValueRef);

pub struct Compiler<'a> {
    source: &'a Source<'a>,
    scanner: Peekable<Scanner<'a>>,
    identifiers: IdentifierTable<'a>,
    extra_bindings: Vec<Binding<'a>>
}

impl <'a> Compiler<'a> {

    pub fn default_bindings() -> Vec<Binding<'static>> {
        vec![
            ("True", builtin::TRUE.clone()),
            ("False", builtin::FALSE.clone()),
            ("And", builtin::AND.clone()),
            ("Or", builtin::OR.clone()),
            ("Not", builtin::NOT.clone()),
            ("If", builtin::IF.clone()),
            ("Unit", builtin::UNIT.clone()),
            ("PrintLn", builtin::PRINTLN.clone()),
            ("Print", builtin::PRINT.clone()),
            ("Is", builtin::IS.clone()),
            ("IsNot", builtin::ISNOT.clone()),
            ("Add", builtin::ADD.clone()),
            ("Sub", builtin::SUB.clone()),
            ("Mul", builtin::MUL.clone()),
            ("Div", builtin::DIV.clone()),
            ("Tuple", builtin::TUPLE.clone()),
            ("TFirst", builtin::TFIRST.clone()),
            ("TSecond", builtin::TSECOND.clone()),
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

    pub fn compile(mut self) -> ExprResult {
        for (ident, _) in &self.extra_bindings {
            self.identifiers.push(ident);
        }

        let mut expr = self.expression()?;

        for (_, provider) in self.extra_bindings.iter().rev() {
            let function = Expr::Fn(expr).default_ref();
            expr = Expr::Call(function, Expr::Value(provider.clone()).default_ref()).default_ref();
        }

        self.match_consume(TokenType::Eof, Error::ExpectedEofAfterExpression)?;

        Ok(expr)
    }

    fn expression(&mut self) -> ExprResult {
        self.then_expression()
    }

    fn let_expression(&mut self) -> ExprResult {
        if self.matches(TokenType::Let).is_none() {
            return self.call();
        }

        let variable_name = self.try_identifier(Error::ExpectedIdentifierAsVariableName)?;

        let be_token = self.match_consume(TokenType::Be, Error::ExpectedBeInAssignment)?;

        let value = self.then_expression()?;

        let in_token = self.match_consume(TokenType::In, Error::ExpectedInAfterAssignment)?;

        // After the initiliazer is finished, the identifier is pushed, so that it isnt available in the initializer
        self.identifiers.push(variable_name);

        let body = self.then_expression()?;

        self.identifiers.pop();

        let function = Expr::Fn(body).annotate(in_token);
        
        Ok(Expr::Call(function, value).annotate(be_token))
    }

    fn then_expression(&mut self) -> ExprResult {
        let mut lhs = self.let_expression()?;

        while let Some(then_token) = self.matches(TokenType::Then) {
            // The value here is discarded, then as a variable is never accessable as it is a keyword
            self.identifiers.push("then");
            let rhs = self.let_expression()?;
            self.identifiers.pop();
            lhs = Expr::Call(lhs, Expr::Fn(rhs).annotate(then_token)).annotate(then_token);
        }

        Ok(lhs)
    }

    fn call(&mut self) -> ExprResult {
        let mut lhs = self.value()?;

        while let Some(of_token) = self.matches(TokenType::Of) {
            let rhs = self.value()?;
            lhs = Expr::Call(lhs, rhs).annotate(of_token);
        }

        Ok(lhs)
    }

    fn value(&mut self) -> ExprResult {
        let token = self.consume();
        match token.token_type {
            TokenType::Do => self.block(),
            TokenType::Fn => self.function(token),
            TokenType::String(is_terminated) => {
                let lexeme = self.source.lexeme(&token);
                match lexeme {
                    "string" => Ok(self.string_of("", token)),
                    _ if is_terminated && lexeme.len() == 7 => Ok(self.string_of("", token)),
                    _ if is_terminated => Ok(self.string_of(&lexeme[4..lexeme.len() - 4], token)),
                    _ => Ok(self.string_of(&lexeme[4..], token)),
                }
            },
            TokenType::Number(num) => Ok(Expr::Value(Value::Number(num).new_ref()).annotate(token)),
            TokenType::Identifier => {
                self.identifiers.distance_from_top(self.source.lexeme(&token)).map(|ident| {
                    Expr::Identifier(ident).annotate(token)
                }).map_err(|e| e.annotate(token))
            },
            _ => Err(Error::ExpectedExpressionFound(token).annotate(token))
        }
    }

    fn string_of(&mut self, string: &str, token: Token) -> ExprRef {
        Expr::Value(Value::String(string.to_string()).new_ref()).annotate(token)
    }

    fn block(&mut self) -> ExprResult {
        let expr = self.expression()?;
        self.match_consume(TokenType::End, Error::ExpectedEndAfterDoBlock)?;
        Ok(expr)
    }

    fn function(&mut self, fn_token: Token) -> ExprResult {
        let mut arguments = vec![self.try_identifier(Error::ExpectedIdentifierAsFunctionArgument)?];

        while let Some(token) = self.matches(TokenType::Identifier) {
            arguments.push(self.source.lexeme(&token));
        }

        for identifier in &arguments {
            self.identifiers.push(identifier);
        }

        self.match_consume(TokenType::Do, Error::ExpectedDoAsFunctionBody)?;

        let body = self.block()?;

        let mut expr = body;

        for _ in 0..arguments.len() {
            expr = Expr::Fn(expr).annotate(fn_token);
            self.identifiers.pop();
        }

        Ok(expr)
    }

    fn try_identifier(&mut self, error: Error) -> Result<&'a str, AnnotatedError> {
        let identifier = self.match_consume(TokenType::Identifier, error)?;

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

    fn match_consume(&mut self, token: TokenType, error: Error) -> Result<Token, AnnotatedError> {
        let error = error.annotate(*self.peek());
        self.matches(token).ok_or(error)
    }
}