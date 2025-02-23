use std::rc::Rc;

use crate::{environment::{EnvRef, Environment}, error::Error, expr::{Expr, ExprRef}, source::{IdentifierTable, Source}, value::Value};

pub type ValueResult = Result<Value, Error>;

pub struct Interpreter<'a> {
    environment: EnvRef,
    expr: ExprRef,
    source: &'a Source<'a>,
    identifiers: IdentifierTable<'a>
}

impl <'a> Interpreter<'a> {

    pub fn interpret(&mut self) -> ValueResult {
        self.evaluate(Rc::clone(&self.expr))
    }

    fn evaluate(&mut self, expr: ExprRef) -> ValueResult {
        match expr.as_ref() {
            Expr::Then(left, right) => {
                let left = self.evaluate(ExprRef::clone(left))?;

                self.call(left, Value::Unit)?;

                self.evaluate(ExprRef::clone(right))
            },
            Expr::Identifier(ident) => Ok(self.environment.from_top(*ident).clone()),
            Expr::Call(lhs, rhs) => {
                let lhs = self.evaluate(ExprRef::clone(lhs))?;
                let rhs = self.evaluate(ExprRef::clone(rhs))?;
                self.call(lhs, rhs)
            },
            Expr::String(ref s) => Ok(Value::String(Rc::new(s.clone()))),
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Value(v) => Ok(*v.clone()),
            Expr::Fn(body) => Ok(Value::Fn(ExprRef::clone(body), EnvRef::clone(&self.environment))),
        }
    }

    fn call(&mut self, lhs: Value, rhs: Value) -> ValueResult {
        match lhs {
            Value::Fn(expr, env) => self.evaluate_fn(expr, env, rhs),
            Value::Builtin(function) => function(rhs, EnvRef::clone(&self.environment)),
            _ => { Err(Error::ValueNotCallable(lhs)) }
        }
    }

    fn evaluate_fn(&mut self, expr: ExprRef, environment: EnvRef, value: Value) -> ValueResult {
        let previous = Rc::clone(&self.environment);
        self.environment = Environment::extend(environment, value);
        let return_value = self.evaluate(expr);
        self.environment = previous;

        return_value
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