use std::{fmt::{Debug, Display}, sync::{Arc, RwLock}};

use crate::{environment::EnvRef, error::Error, expr::ExprRef};

pub type ValueRef = Arc<Value>;

#[derive(Debug, Clone)]
pub enum BuiltIn {
    Print,
    PrintLn,
    Is,
    IsOf(ValueRef),
    IsNot,
    IsNotOf(ValueRef),
    Add,
    AddOf(f64),
    Sub,
    SubOf(f64),
    Mul,
    MulOf(f64),
    Div,
    DivOf(f64)
}

pub enum Value {
    Number(f64),
    String(String),
    Unit,
    Lazy(ExprRef, EnvRef, Arc<RwLock<Option<ValueRef>>>),
    Fn(ExprRef, EnvRef),
    Builtin(BuiltIn)
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
            Value::Builtin(b) => f.write_fmt(format_args!("Builtin {b:?}")),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Unit => write!(f, "Unit"),
            Self::Lazy(arg0, arg1, arg2) => f.debug_tuple("Lazy").field(arg0).field(arg1).field(arg2).finish(),
            Self::Fn(arg0, arg1) => f.debug_tuple("Fn").field(arg0).field(arg1).finish(),
            Self::Builtin(_) => f.debug_tuple("Builtin").field(&"Function").finish(),
        }
    }
}