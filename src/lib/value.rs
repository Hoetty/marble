use std::{fmt::Display, rc::Rc};

use crate::{environment::EnvRef, expr::ExprRef, interpreter::ValueResult};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(Rc<String>),
    Unit,
    Fn(ExprRef, EnvRef),
    Builtin(Rc<dyn Fn(Value, EnvRef) -> ValueResult>)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{n}")),
            Value::String(s) => f.write_fmt(format_args!("{s}")),
            Value::Unit => f.write_str("Unit"),
            Value::Fn(_, _) => f.write_str("Function"),
            Value::Builtin(_) => f.write_str("Builtin Function"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Unit, Self::Unit) => true,
            _ => false,
        }
    }
}