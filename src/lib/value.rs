use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{environment::EnvRef, error::Error, expr::ExprRef, interpreter::ValueResult};

pub type ValueRef = Rc<Value>;

pub enum Value {
    Number(f64),
    String(String),
    Unit,
    Lazy(ExprRef, EnvRef, RefCell<Option<ValueRef>>),
    Fn(ExprRef, EnvRef),
    Builtin(Box<dyn Fn(ValueRef, EnvRef) -> ValueResult>)
}

impl Value {
    pub fn number_for_operator(&self, operator: &'static str) -> Result<f64, Error> {
        match self {
            Value::Number(f) => Ok(*f),
            _ => Err(Error::ArgumentToOperatorMustBeANumber(operator))
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Unit => "Unit",
            Value::Lazy(_, _, _) => "Lazy",
            Value::Fn(_, _) => "Function",
            Value::Builtin(_) => "Builtin",
        }
    }

    #[inline]
    pub fn new_ref(self) -> ValueRef {
        ValueRef::new(self)
    }

}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{n}")),
            Value::String(s) => f.write_fmt(format_args!("{s}")),
            Value::Unit => f.write_str("Unit"),
            Value::Lazy(_, _, _) => f.write_str("Lazy"),
            Value::Fn(_, _) => f.write_str("Function"),
            Value::Builtin(_) => f.write_str("Builtin Function"),
        }
    }
}