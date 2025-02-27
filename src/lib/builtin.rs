use crate::environment::Environment;
use crate::unit;
use crate::{call, expr::{Expr, ExprRef}, fun, fun_val, identifier, value::{Value, ValueRef}};

macro_rules! builtin_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $result: expr) => {
        ValueRef::new(Value::Builtin(Box::new(move |$lhs, _| {
            Ok(Value::Builtin(Box::new(move |$rhs, $env| {
                $result
            })).new_ref())
        })))
    };
}

pub fn get_print() -> ValueRef {
    ValueRef::new(Value::Builtin(Box::new(move |lhs, _| {
        match &lhs.as_ref() {
            Value::Number(n) => print!("{n}"),
            Value::String(s) => print!("{s}"),
            Value::Unit => print!("Unit"),
            Value::Lazy(_, _, _) => panic!("Lazy passed to print"),
            Value::Fn(_, _) => print!("Function"),
            Value::Builtin(_) => print!("Builtin Function"),
        };

        Ok(fun_val!(call!(identifier!(0), unit!())))
    })))
}

pub fn get_println() -> ValueRef {
    ValueRef::new(Value::Builtin(Box::new(move |lhs, _| {
        match &lhs.as_ref() {
            Value::Number(n) => println!("{n}"),
            Value::String(s) => println!("{s}"),
            Value::Unit => println!("Unit"),
            Value::Lazy(_, _, _) => panic!("Lazy passed to print"),
            Value::Fn(_, _) => println!("Function"),
            Value::Builtin(_) => println!("Builtin Function"),
        };

        Ok(fun_val!(call!(identifier!(0), unit!())))
    })))
}

pub fn get_is() -> ValueRef {
    builtin_binary!(lhs, rhs, env, Ok(if match (lhs.as_ref(), rhs.as_ref()) {
        (Value::Number(l0), Value::Number(r0)) => l0 == r0,
        (Value::String(l0), Value::String(r0)) => l0 == r0,
        (Value::Unit, Value::Unit) => true,
        _ => false,
    } { 
        env.true_fn.clone()
    } else { 
        env.false_fn.clone()
    }))
}

pub fn get_is_not() -> ValueRef {
    builtin_binary!(lhs, rhs, env, Ok(if match (lhs.as_ref(), rhs.as_ref()) {
        (Value::Number(l0), Value::Number(r0)) => l0 == r0,
        (Value::String(l0), Value::String(r0)) => l0 == r0,
        (Value::Unit, Value::Unit) => true,
        _ => false,
    } { 
        env.false_fn.clone()
    } else { 
        env.true_fn.clone()
    }))
}

macro_rules! builtin_number_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $name: literal, $result: expr) => {{
        Value::Builtin(Box::new(move |$lhs, _| {
            let $lhs = $lhs.number_for_operator($name)?;
            Ok(Value::Builtin(Box::new(move |$rhs, $env| {
                let $rhs = $rhs.number_for_operator($name)?;
                $result
            })).new_ref())
        })).new_ref()
    }};
}

pub fn get_add() -> ValueRef {
    builtin_number_binary!(lhs, rhs, _env, "Add", Ok(Value::Number(lhs + rhs).new_ref()))
}

pub fn get_sub() -> ValueRef {
    builtin_number_binary!(lhs, rhs, _env, "Sub", Ok(Value::Number(lhs - rhs).new_ref()))
}

pub fn get_mul() -> ValueRef {
    builtin_number_binary!(lhs, rhs, _env, "Mul", Ok(Value::Number(lhs * rhs).new_ref()))
}

pub fn get_div() -> ValueRef {
    builtin_number_binary!(lhs, rhs, _env, "Div", Ok(Value::Number(lhs / rhs).new_ref()))
}

pub fn get_true() -> ValueRef {
    fun_val!(fun!(identifier!(1)))
}

pub fn get_false() -> ValueRef {
    fun_val!(fun!(identifier!(0)))
}

pub fn get_not() -> ValueRef {
    fun_val!(fun!(fun!(
        call!(
            call!(identifier!(2), identifier!(0)), 
            identifier!(1)
        )
    )))
}

pub fn get_if() -> ValueRef {
    fun_val!(fun!(fun!(
        call!(
            call!(identifier!(2), identifier!(1)), 
            identifier!(0)
        )
    )))
}

pub fn get_or() -> ValueRef {
    fun_val!(fun!(
        call!(
            call!(identifier!(1), identifier!(1)), 
            identifier!(0)
        )
    ))
}

pub fn get_and() -> ValueRef {
    fun_val!(fun!(
        call!(
            call!(identifier!(1), identifier!(0)), 
            identifier!(1)
        )
    ))
}