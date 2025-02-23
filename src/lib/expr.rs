use std::rc::Rc;

use crate::value::Value;

pub type IdentRef = usize;
pub type ExprRef = Rc<Expr>;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Then(ExprRef, ExprRef),
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    String(String),
    Number(f64),
    Value(Box<Value>),
    Fn(ExprRef)
}