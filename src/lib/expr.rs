use std::rc::Rc;

pub type IdentRef = usize;
pub type ExprRef = Rc<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Then(ExprRef, ExprRef),
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    String(String),
    Number(f64),
    Fn(IdentRef, ExprRef)
}