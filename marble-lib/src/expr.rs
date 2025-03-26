use std::{ops::Deref, sync::Arc};

use crate::{token::Token, value::ValueRef};

pub type IdentRef = usize;
pub type ExprRef = Arc<AnnotatedExpr>;

#[derive(Debug, Clone)]
pub struct AnnotatedExpr {
    pub expr: Expr,
    pub token: Token,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Identifier(IdentRef),
    Call(ExprRef, ExprRef),
    Value(ValueRef),
    Fn(ExprRef),
}

impl Deref for AnnotatedExpr {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        &self.expr
    }
}

impl Expr {
    pub fn default_ref(self) -> ExprRef {
        self.annotate(Token::default())
    }

    pub fn annotate(self, token: Token) -> ExprRef {
        AnnotatedExpr::new(self, token).new_ref()
    }
}

impl AnnotatedExpr {
    pub fn new_ref(self) -> ExprRef {
        ExprRef::new(self)
    }

    pub fn new(expr: Expr, token: Token) -> AnnotatedExpr {
        AnnotatedExpr { expr, token }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}
