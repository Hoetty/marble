use std::rc::Rc;

use crate::environment::Value;

pub fn print(value: Value) -> Value {
    Value::Builtin(Rc::new(move |_| {
        println!("{value}");
        Value::Unit
    }))
}