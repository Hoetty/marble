use std::{cell::RefCell, rc::Rc};

use crate::{environment::{EnvRef, Environment}, error::Error, expr::{Expr, ExprRef}, source::{IdentifierTable, Source}, value::{Value, ValueRef}};

pub type ValueResult = Result<ValueRef, Error>;

pub struct Interpreter<'a> {
    environment: EnvRef,
    expr: ExprRef,
    source: &'a Source<'a>,
    identifiers: IdentifierTable<'a>
}

impl <'a> Interpreter<'a> {

    pub fn interpret(&mut self) -> ValueResult {
        let value = self.evaluate(Rc::clone(&self.expr))?;
        self.force(value)
    }

    fn evaluate(&mut self, expr: ExprRef) -> ValueResult {
        match expr.as_ref() {
            Expr::Call(_, _) | Expr::Then(_, _) => {
                Ok(Value::Lazy(expr, EnvRef::clone(&self.environment), RefCell::new(None)).new_ref())
            },
            Expr::Identifier(ident) => Ok(self.environment.from_top(*ident).clone()),
            Expr::String(ref s) => Ok(Value::String(s.clone()).new_ref()),
            Expr::Number(n) => Ok(Value::Number(*n).new_ref()),
            Expr::Value(v) => Ok(ValueRef::clone(v)),
            Expr::Fn(body) => Ok(Value::Fn(ExprRef::clone(body), EnvRef::clone(&self.environment)).new_ref()),
        }
    }

    fn call(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueResult {
        let lhs = self.force(lhs)?;
        match lhs.as_ref() {
            Value::Fn(expr, env) => self.evaluate_fn(ExprRef::clone(expr), EnvRef::clone(env), rhs),
            Value::Builtin(function) => function(self.force(rhs)?, EnvRef::clone(&self.environment)),
            _ => { Err(Error::ValueNotCallable(lhs)) }
        }
    }

    fn evaluate_fn(&mut self, expr: ExprRef, environment: EnvRef, value: ValueRef) -> ValueResult {
        let previous = Rc::clone(&self.environment);
        self.environment = Environment::extend(environment, value);
        let return_value = self.evaluate(expr);
        self.environment = previous;

        return_value
    }

    fn force(&mut self, mut value: ValueRef) -> ValueResult {
        while let Value::Lazy(expr, env, possible) = value.as_ref() {
            let result = match *possible.borrow() {
                Some(ref v) => ValueRef::clone(v),
                None =>  match expr.as_ref() {
                    Expr::Call(lhs, rhs) => {
                        let previous_env = EnvRef::clone(&self.environment);
                        self.environment = EnvRef::clone(env);

                        let lhs = self.evaluate(ExprRef::clone(lhs))?;
                        let rhs = self.evaluate(ExprRef::clone(rhs))?;

                        let result = self.call(lhs, rhs)?;

                        self.environment = previous_env;

                        result
                    },
                    Expr::Then(lhs, rhs) => {
                        let previous_env = EnvRef::clone(&self.environment);
                        self.environment = EnvRef::clone(env);

                        let lhs = self.evaluate(ExprRef::clone(lhs))?;
                        let rhs = self.evaluate(ExprRef::clone(rhs))?;

                        self.call(lhs, Value::Unit.new_ref())?;

                        self.environment = previous_env;

                        rhs
                    },
                    _ => self.evaluate(ExprRef::clone(expr))?
                }
            };

            // Save the value for later, so it isnt computed again
            *possible.try_borrow_mut().map_err(|_| Error::ValueDependsOnItself)? = Some(ValueRef::clone(&result));
            value = result;
        }

        Ok(value)
    }

    pub fn new(expr: ExprRef, source: &'a Source<'a>, identifiers: IdentifierTable<'a>) -> Self {
        Self { 
            environment: Environment::root(), 
            expr, 
            source,
            identifiers
        }
    }
}