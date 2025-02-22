use std::rc::Rc;

use crate::{ast::{Ast, Expr, ExprRef, IdentRef}, builtin, environment::{EnvRef, Environment, Value}, source::{IdentifierTable, Source}};

pub struct Interpreter<'a> {
    environment: EnvRef,
    ast: Ast,
    source: &'a Source<'a>,
    identifiers: IdentifierTable<'a>
}

impl <'a> Interpreter<'a> {

    fn default_environment(identifiers: &mut IdentifierTable) -> EnvRef {
        let mut environment = Environment::root();

        environment = Environment::extend(environment, identifiers.reference("Print"), Value::Builtin(Rc::new(builtin::print)));

        environment
    }

    pub fn interpret(&mut self) -> Value {
        self.evaluate(self.ast.exprs.len() - 1)
    }

    fn evaluate(&mut self, expr: ExprRef) -> Value {
        match *self.ast.expr(expr) {
            Expr::Then(left, right) => {
                let left = self.evaluate(left);

                self.try_call(left, Value::Unit);

                self.evaluate(right)
            },
            Expr::Identifier(ident) => match self.environment.find(ident) {
                Some(value) => value,
                None => panic!("{} is not defined", self.identifiers.name(ident)),
            },
            Expr::Call(lhs, rhs) => {
                let lhs = self.evaluate(lhs);
                let rhs = self.evaluate(rhs);
                self.try_call(lhs, rhs).expect("Can't call non fn lhs in of expression")
            },
            Expr::String(ref s) => Value::String(s.clone()),
            Expr::Number(n) => Value::Number(n),
            Expr::Fn(argument, body) => Value::Fn(argument, body, Rc::clone(&self.environment)),
        }
    }

    fn try_call(&mut self, lhs: Value, rhs: Value) -> Option<Value> {
        match lhs {
            Value::Fn(ident, expr, env) => Some(self.evaluate_fn(ident, expr, env, rhs)),
            Value::Builtin(function) => Some(function(rhs)),
            _ => { None }
        }
    }

    fn evaluate_fn(&mut self, ident: IdentRef, expr: ExprRef, environment: EnvRef, value: Value) -> Value {
        let previous = Rc::clone(&self.environment);
        self.environment = Environment::extend(environment, ident, value);
        let return_value = self.evaluate(expr);
        self.environment = previous;

        return_value
    }

    pub fn new(ast: Ast, source: &'a Source<'a>, mut identifiers: IdentifierTable<'a>) -> Self {
        Self { 
            environment: Self::default_environment(&mut identifiers), 
            ast, 
            source,
            identifiers
        }
    }

}