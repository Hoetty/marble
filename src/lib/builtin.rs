use std::rc::Rc;

use crate::{environment::Environment, expr::{Expr, ExprRef}, interpreter::ValueResult, source::IdentifierTable, value::Value};

macro_rules! builtin_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $result: expr) => {
        ExprRef::new(Expr::Value(Box::new(Value::Builtin(Rc::new(move |$lhs, _| {
            Ok(Value::Builtin(Rc::new(move |$rhs, $env| {
                $result
            })))
        })))))
    };
}

pub fn get_print() -> ExprRef {
    builtin_binary!(lhs, _rhs, _env, Ok({
        println!("{lhs}");
        Value::Unit
    }))
}

pub fn get_is() -> ExprRef {
    builtin_binary!(lhs, rhs, env, Ok(if lhs == rhs { 
        env.from_bottom(0)
    } else { 
        env.from_bottom(1)
    }))
}

pub fn get_true() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Identifier(1))
        ))
    ))
}

pub fn get_false() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Identifier(0))
        ))
    ))
}

pub fn get_not() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Fn(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Call(
                        ExprRef::new(Expr::Identifier(2)),
                        ExprRef::new(Expr::Identifier(0))
                    )),
                    ExprRef::new(Expr::Identifier(1))
                ))
            ))
        )),
    ))
}

pub fn get_if() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Fn(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Call(
                        ExprRef::new(Expr::Identifier(2)),
                        ExprRef::new(Expr::Identifier(1))
                    )),
                    ExprRef::new(Expr::Identifier(0))
                ))
            ))
        )),
    ))
}

pub fn get_or() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Call(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Identifier(1)),
                    ExprRef::new(Expr::Identifier(1))
                )),
                ExprRef::new(Expr::Identifier(0))
            ))
        )),
    ))
}

pub fn get_and() -> ExprRef {
    ExprRef::new(Expr::Fn(
        ExprRef::new(Expr::Fn(
            ExprRef::new(Expr::Call(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Identifier(1)),
                    ExprRef::new(Expr::Identifier(0))
                )),
                ExprRef::new(Expr::Identifier(1))
            ))
        )),
    ))
}