use std::rc::Rc;

use crate::{environment::{Environment, Value}, expr::{Expr, ExprRef}, source::IdentifierTable};

pub fn print(value: Value) -> Value {
    Value::Builtin(Rc::new(move |_| {
        println!("{value}");
        Value::Unit
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