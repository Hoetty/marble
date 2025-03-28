#[macro_export]
macro_rules! fun {
    ($body: expr) => {
        Expr::Fn($body).default_ref()
    };
}

#[macro_export]
macro_rules! fun_val {
    ($body: expr) => {
        Value::Fn($body, Environment::root()).new_ref()
    };
}

#[macro_export]
macro_rules! identifier {
    ($ident: expr) => {
        Expr::Identifier($ident).default_ref()
    };
}

#[macro_export]
macro_rules! call {
    ($lhs: expr, $rhs: expr) => {
        Expr::Call($lhs, $rhs).default_ref()
    };
}

#[macro_export]
macro_rules! unit {
    () => {
        Expr::Value(builtin::UNIT.clone()).default_ref()
    };
}
