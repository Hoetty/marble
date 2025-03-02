use crate::{builtin, value::{Value, ValueRef}};

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
            true_fn: builtin::get_true(), 
            false_fn: builtin::get_false(), 
            not: builtin::get_not(),
            and: builtin::get_and(),
            or: builtin::get_or(),
            if_fn: builtin::get_if(),
            unit: ValueRef::new(Value::Unit),
            println: builtin::get_println(),
            print: builtin::get_print(),
            is: builtin::get_is(),
            is_not: builtin::get_is_not(),
            add: builtin::get_add(),
            sub: builtin::get_sub(),
            mul: builtin::get_mul(),
            div: builtin::get_div(),
        }
    }
}