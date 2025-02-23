use std::rc::Rc;

use crate::{environment::{EnvRef, Environment}, expr::{Expr, ExprRef}, interpreter::ValueResult, source::IdentifierTable, value::Value};

pub fn print(value: Value, _: EnvRef) -> ValueResult {
    Ok(Value::Builtin(Rc::new(move |_, _| {
        println!("{value}");
        Ok(Value::Unit)
    })))
}

macro_rules! builtin_binary {
    ($lhs: ident, $rhs: ident, $env: ident, $result: expr) => {
        Value::Builtin(Rc::new(move |$lhs, _| {
            Ok(Value::Builtin(Rc::new(move |$rhs, $env| {
                $result
            })))
        }))
    };
}

pub fn get_is(identifiers: &mut IdentifierTable) -> Value {
    let true_ref = identifiers.reference("True");
    let false_ref = identifiers.reference("False");

    builtin_binary!(lhs, rhs, env, Ok(if lhs == rhs { 
        env.find(true_ref).expect("True must be defined") 
    } else { 
        env.find(false_ref).expect("False must be defined") 
    }))
}

pub fn get_true(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("L"), 
        ExprRef::new(Expr::Fn(
            identifiers.reference("R"), 
            ExprRef::new(Expr::Identifier(identifiers.reference("L")))
        )), 
        Environment::root()
    )
}

pub fn get_false(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("L"),
        ExprRef::new(Expr::Fn(
            identifiers.reference("R"),
            ExprRef::new(Expr::Identifier(identifiers.reference("R")))
        )),
        Environment::root()
    )
}

pub fn get_not(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("Predicate"),
        ExprRef::new(Expr::Fn(
            identifiers.reference("L"),
            ExprRef::new(Expr::Fn(
                identifiers.reference("R"),
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Call(
                        ExprRef::new(Expr::Identifier(identifiers.reference("Predicate"))),
                        ExprRef::new(Expr::Identifier(identifiers.reference("R")))
                    )),
                    ExprRef::new(Expr::Identifier(identifiers.reference("L")))
                ))
            ))
        )),
        Environment::root()
    )
}

pub fn get_if(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("Predicate"),
        ExprRef::new(Expr::Fn(
            identifiers.reference("L"),
            ExprRef::new(Expr::Fn(
                identifiers.reference("R"),
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Call(
                        ExprRef::new(Expr::Identifier(identifiers.reference("Predicate"))),
                        ExprRef::new(Expr::Identifier(identifiers.reference("L")))
                    )),
                    ExprRef::new(Expr::Identifier(identifiers.reference("R")))
                ))
            ))
        )),
        Environment::root()
    )
}

pub fn get_or(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("L"),
        ExprRef::new(Expr::Fn(
            identifiers.reference("R"),
            ExprRef::new(Expr::Call(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Identifier(identifiers.reference("L"))),
                    ExprRef::new(Expr::Identifier(identifiers.reference("L")))
                )),
                ExprRef::new(Expr::Identifier(identifiers.reference("R")))
            ))
        )),
        Environment::root()
    )
}

pub fn get_and(identifiers: &mut IdentifierTable) -> Value {
    Value::Fn(
        identifiers.reference("L"),
        ExprRef::new(Expr::Fn(
            identifiers.reference("R"),
            ExprRef::new(Expr::Call(
                ExprRef::new(Expr::Call(
                    ExprRef::new(Expr::Identifier(identifiers.reference("L"))),
                    ExprRef::new(Expr::Identifier(identifiers.reference("R")))
                )),
                ExprRef::new(Expr::Identifier(identifiers.reference("L")))
            ))
        )),
        Environment::root()
    )
}