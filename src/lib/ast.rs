#[derive(Clone, Default, Debug)]
pub struct Ast {
    pub exprs: Vec<Expr>,
}

pub type ExprRef = usize;
pub type IdentRef = usize;

impl Ast {

    pub fn push_expr(&mut self, expr: Expr) -> ExprRef {
        let index = self.exprs.len();
        self.exprs.push(expr);
        index
    }

    #[inline]
    pub fn expr(&self, index: ExprRef) -> &Expr {
        &self.exprs[index]
    }

    pub fn new() -> Ast {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Then(ExprRef, ExprRef),
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    String(String),
    Number(f64),
    Fn(IdentRef, ExprRef)
}