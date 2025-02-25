use std::rc::Rc;

use crate::{expr::{Expr, ExprRef}, value::{Value, ValueRef}};

macro_rules! builtin_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $result: expr) => {
        ExprRef::new(Expr::Value(ValueRef::new(Value::Builtin(Box::new(move |$lhs, _| {
            Ok(Value::Builtin(Box::new(move |$rhs, $env| {
                $result
            })).new_ref())
        })))))
    };
}

pub fn get_print() -> ExprRef {
    builtin_binary!(lhs, _rhs, _env, Ok({
        match &lhs.as_ref() {
            Value::Number(n) => println!("{n}"),
            Value::String(s) => println!("{s}"),
            Value::Unit => println!("Unit"),
            Value::Lazy(_, _, _) => panic!("Lazy passed to print"),
            Value::Fn(_, _) => println!("Function"),
            Value::Builtin(_) => println!("Builtin Function"),
        };
        Value::Unit.new_ref()
    }))
}

pub fn get_is() -> ExprRef {
    builtin_binary!(lhs, rhs, env, Ok(if match (lhs.as_ref(), rhs.as_ref()) {
        (Value::Number(l0), Value::Number(r0)) => l0 == r0,
        (Value::String(l0), Value::String(r0)) => l0 == r0,
        (Value::Unit, Value::Unit) => true,
        _ => false,
    } { 
        env.from_bottom(0)
    } else { 
        env.from_bottom(1)
    }))
}

pub fn get_is_not() -> ExprRef {
    builtin_binary!(lhs, rhs, env, Ok(if match (lhs.as_ref(), rhs.as_ref()) {
        (Value::Number(l0), Value::Number(r0)) => l0 == r0,
        (Value::String(l0), Value::String(r0)) => l0 == r0,
        (Value::Unit, Value::Unit) => true,
        _ => false,
    } { 
        env.from_bottom(1)
    } else { 
        env.from_bottom(0)
    }))
}

macro_rules! builtin_number_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $name: literal, $result: expr) => {{
        ExprRef::new(Expr::Value(Value::Builtin(Box::new(move |$lhs, _| {
            let $lhs = $lhs.number_for_operator($name)?;
            Ok(Value::Builtin(Box::new(move |$rhs, $env| {
                let $rhs = $rhs.number_for_operator($name)?;
                $result
            })).new_ref())
        })).new_ref()))
    }};
}

pub fn get_add() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Add", Ok(Value::Number(lhs + rhs).new_ref()))
}

pub fn get_sub() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Sub", Ok(Value::Number(lhs - rhs).new_ref()))
}

pub fn get_mul() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Mul", Ok(Value::Number(lhs * rhs).new_ref()))
}

pub fn get_div() -> ExprRef {
    builtin_number_binary!(lhs, rhs, _env, "Div", Ok(Value::Number(lhs / rhs).new_ref()))
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