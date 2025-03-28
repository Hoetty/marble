use std::sync::LazyLock;

use crate::environment::Environment;
use crate::value::BuiltIn;
use crate::{
    call,
    expr::Expr,
    fun, fun_val, identifier,
    value::{Value, ValueRef},
};

macro_rules! value {
    ($name: ident, $expr: expr) => {
        pub static $name: LazyLock<ValueRef> = LazyLock::new(|| $expr);
    };
}

macro_rules! builtin {
    ($name: ident, $builtin: ident) => {
        value!($name, ValueRef::new(Value::Builtin(BuiltIn::$builtin)));
    };
}

builtin!(PRINT, Print);
builtin!(PRINTLN, PrintLn);
builtin!(ADD, Add);
builtin!(SUB, Sub);
builtin!(MUL, Mul);
builtin!(DIV, Div);

builtin!(IS, Is);
builtin!(ISNOT, IsNot);

value!(UNIT, Value::Unit.new_ref());

value!(TRUE, fun_val!(fun!(identifier!(1))));
value!(FALSE, fun_val!(fun!(identifier!(0))));

value!(
    NOT,
    fun_val!(fun!(fun!(call!(
        call!(identifier!(2), identifier!(0)),
        identifier!(1)
    ))))
);

value!(
    IF,
    fun_val!(fun!(fun!(call!(
        call!(identifier!(2), identifier!(1)),
        identifier!(0)
    ))))
);

value!(
    OR,
    fun_val!(fun!(call!(
        call!(identifier!(1), identifier!(1)),
        identifier!(0)
    )))
);

value!(
    AND,
    fun_val!(fun!(call!(
        call!(identifier!(1), identifier!(0)),
        identifier!(1)
    )))
);

value!(
    TUPLE,
    fun_val!(fun!(fun!(call!(
        call!(identifier!(0), identifier!(2)),
        identifier!(1)
    ))))
);

value!(
    TFIRST,
    fun_val!(call!(
        identifier!(0),
        Expr::Value(TRUE.clone()).default_ref()
    ))
);

value!(
    TSECOND,
    fun_val!(call!(
        identifier!(0),
        Expr::Value(FALSE.clone()).default_ref()
    ))
);
