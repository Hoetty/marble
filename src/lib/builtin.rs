use std::rc::Rc;

use crate::{expr::{Expr, ExprRef}, value::Value};

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

pub fn get_is_not() -> ExprRef {
    builtin_binary!(lhs, rhs, env, Ok(if lhs == rhs { 
        env.from_bottom(1)
    } else { 
        env.from_bottom(0)
    }))
}

macro_rules! builtin_number_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $name: literal, $result: expr) => {{
        ExprRef::new(Expr::Value(Box::new(Value::Builtin(Rc::new(move |$lhs, _| {
            let $lhs = $lhs.number_for_operator($name)?;
            Ok(Value::Builtin(Rc::new(move |$rhs, $env| {
                let $rhs = $rhs.number_for_operator($name)?;
                $result
            })))
        })))))
    }};
}

pub fn get_add() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Add", Ok(Value::Number(lhs + rhs)))
}

pub fn get_sub() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Sub", Ok(Value::Number(lhs - rhs)))
}

pub fn get_mul() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Mul", Ok(Value::Number(lhs * rhs)))
}

pub fn get_div() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Div", Ok(Value::Number(lhs / rhs)))
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