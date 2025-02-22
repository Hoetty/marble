use std::{fmt::Display, rc::Rc};

use crate::expr::{ExprRef, IdentRef};

pub type EnvRef = Rc<Environment>;

pub enum Environment {
    Value {
        ident: IdentRef,
        value: Value,
        parent: EnvRef,
    },
    Root
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Unit,
    Fn(IdentRef, ExprRef, EnvRef),
    Builtin(Rc<dyn Fn(Value) -> Value>)
}

impl  Environment {
    pub fn extend(environment: EnvRef, ident: IdentRef, value: Value) -> EnvRef {
        Rc::new(Environment::Value { ident, value, parent: Rc::clone(&environment) })
    }

    pub fn pop(environment: &EnvRef) -> EnvRef {
        match Rc::as_ref(environment) {
            Environment::Value { ident: _, value: _, parent } => Rc::clone(parent),
            Environment::Root => panic!("Popped the root Environment"),
        }
    }

    pub fn clone(environment: &EnvRef) -> EnvRef {
        Rc::clone(environment)
    }

    pub fn root() -> EnvRef {
        Rc::new(Environment::Root)
    }

    pub fn find(&self, ident: IdentRef) -> Option<Value> {
        match self {
            Environment::Value { ident: other, value, parent } => if ident == *other {
                Some(value.clone())
            } else {
                parent.find(ident)
            },
            Environment::Root => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{n}")),
            Value::String(s) => f.write_fmt(format_args!("{s}")),
            Value::Unit => f.write_str("Unit"),
            Value::Fn(_, _, _) => f.write_str("Function"),
            Value::Builtin(_) => f.write_str("Builtin Function"),
        }
    }
}