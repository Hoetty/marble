use std::sync::Arc;

use crate::value::ValueRef;

pub type IdentRef = usize;
pub type ExprRef = Arc<Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    String(String),
    Number(f64),
    Value(ValueRef),
    Fn(ExprRef)
}

impl Expr {

    pub fn new_ref(self) -> ExprRef {
        ExprRef::new(self)
    }

}