#[macro_export]
macro_rules! fun {
    ($body: expr) => {
        ExprRef::new(Expr::Fn($body))
    };
}

#[macro_export]
macro_rules! fun_val {
    ($body: expr) => {
        ValueRef::new(Value::Fn($body, Environment::root()))
    };
}

#[macro_export]
macro_rules! identifier {
    ($ident: expr) => {
        ExprRef::new(Expr::Identifier($ident))
    };
}

#[macro_export]
macro_rules! call {
    ($lhs: expr, $rhs: expr) => {
        ExprRef::new(Expr::Call($lhs, $rhs))
    };
}

#[macro_export]
macro_rules! unit {
    () => {
        ExprRef::new(Expr::Value(builtin::UNIT.clone()))
    };
}