use std::rc::Rc;

use crate::{builtin, environment::{EnvRef, Environment}, error::Error, expr::{Expr, ExprRef, IdentRef}, source::{IdentifierTable, Source}, value::Value};

pub type ValueResult = Result<Value, Error>;

pub struct Interpreter<'a> {
    environment: EnvRef,
    expr: ExprRef,
    source: &'a Source<'a>,
    identifiers: IdentifierTable<'a>
}

impl <'a> Interpreter<'a> {

    fn default_environment(identifiers: &mut IdentifierTable) -> EnvRef {
        let mut environment = Environment::root();

        environment = Environment::extend(environment, identifiers.reference("Print"), Value::Builtin(Rc::new(builtin::print)));
        environment = Environment::extend(environment, identifiers.reference("True"), builtin::get_true(identifiers));
        environment = Environment::extend(environment, identifiers.reference("False"), builtin::get_false(identifiers));
        environment = Environment::extend(environment, identifiers.reference("And"), builtin::get_and(identifiers));
        environment = Environment::extend(environment, identifiers.reference("Or"), builtin::get_or(identifiers));
        environment = Environment::extend(environment, identifiers.reference("Not"), builtin::get_not(identifiers));
        environment = Environment::extend(environment, identifiers.reference("If"), builtin::get_if(identifiers));
        environment = Environment::extend(environment, identifiers.reference("Unit"), Value::Unit);
        environment = Environment::extend(environment, identifiers.reference("Is"), builtin::get_is(identifiers));

        environment
    }

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
            Expr::Identifier(ident) => match self.environment.find(*ident) {
                Some(value) => Ok(value),
                None => Err(Error::IdentifierIsNotDefined(self.identifiers.name(*ident).to_string())),
            },
            Expr::Call(lhs, rhs) => {
                let lhs = self.evaluate(ExprRef::clone(lhs))?;
                let rhs = self.evaluate(ExprRef::clone(rhs))?;
                self.call(lhs, rhs)
            },
            Expr::String(ref s) => Ok(Value::String(s.clone())),
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Fn(argument, body) => Ok(Value::Fn(*argument, ExprRef::clone(body), EnvRef::clone(&self.environment))),
        }
    }

    fn call(&mut self, lhs: Value, rhs: Value) -> ValueResult {
        match lhs {
            Value::Fn(ident, expr, env) => self.evaluate_fn(ident, expr, env, rhs),
            Value::Builtin(function) => function(rhs, EnvRef::clone(&self.environment)),
            _ => { Err(Error::ValueNotCallable(lhs)) }
        }
    }

    fn evaluate_fn(&mut self, ident: IdentRef, expr: ExprRef, environment: EnvRef, value: Value) -> ValueResult {
        let previous = Rc::clone(&self.environment);
        self.environment = Environment::extend(environment, ident, value);
        let return_value = self.evaluate(expr);
        self.environment = previous;

        return_value
    }

    pub fn new(expr: ExprRef, source: &'a Source<'a>, mut identifiers: IdentifierTable<'a>) -> Self {
        Self { 
            environment: Self::default_environment(&mut identifiers), 
            expr, 
            source,
            identifiers
        }
    }
}