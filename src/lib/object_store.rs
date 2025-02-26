use crate::{builtin, environment::Environment, expr::Expr, value::{Value, ValueRef}};

macro_rules! fn_to_val {
    ($expr: expr) => {
        match $expr.as_ref() {
            Expr::Fn(body) => Value::Fn(body.clone(), Environment::root()).new_ref(),
            Expr::Value(v) => v.clone(),
            _ => panic!()
        }
    };
}

#[derive(Clone)]
pub struct ObjectStore {
    pub true_fn: ValueRef,
    pub false_fn: ValueRef,
    pub and: ValueRef,
    pub or: ValueRef,
    pub not: ValueRef,
    pub if_fn: ValueRef,
    pub unit: ValueRef,
    pub println: ValueRef,
    pub print: ValueRef,
    pub is: ValueRef,
    pub is_not: ValueRef,
    pub add: ValueRef,
    pub sub: ValueRef,
    pub mul: ValueRef,
    pub div: ValueRef
}

impl Default for ObjectStore {
    fn default() -> Self {
        Self { 
            true_fn: fn_to_val!(builtin::get_true()), 
            false_fn: fn_to_val!(builtin::get_false()), 
            not: fn_to_val!(builtin::get_not()),
            and: fn_to_val!(builtin::get_and()),
            or: fn_to_val!(builtin::get_or()),
            if_fn: fn_to_val!(builtin::get_if()),
            unit: ValueRef::new(Value::Unit),
            println: fn_to_val!(builtin::get_println()),
            print: fn_to_val!(builtin::get_print()),
            is: fn_to_val!(builtin::get_is()),
            is_not: fn_to_val!(builtin::get_is_not()),
            add: fn_to_val!(builtin::get_add()),
            sub: fn_to_val!(builtin::get_sub()),
            mul: fn_to_val!(builtin::get_mul()),
            div: fn_to_val!(builtin::get_div()),
        }
    }
}