use std::rc::Rc;

use crate::value::ValueRef;

pub type IdentRef = usize;
pub type ExprRef = Rc<Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
    Then(ExprRef, ExprRef),
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    String(String),
    Number(f64),
    Value(ValueRef),
    Fn(ExprRef)
}