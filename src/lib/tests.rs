use crate::{error::Error, evaluate_file_at, value::Value};

pub fn file_name(name: &str) -> String {
    format!("examples/test/{name}.mrbl")
}

macro_rules! make_test {
    ($name: ident, $value: expr) => {
        #[test]
        fn $name() {
            $value;
        }
    };
}

macro_rules! expect_value {
    ($name: ident, $pattern: pat) => {
        make_test!($name, assert!(matches!(evaluate_file_at(&file_name(stringify!($name))).unwrap().as_ref(), $pattern)));
    };
}

macro_rules! expect_error {
    ($name: ident, $pattern: pat) => {
        make_test!($name, assert!(matches!(evaluate_file_at(&file_name(stringify!($name))).err().unwrap(), $pattern)));
    };
}

expect_value!(fact, Value::Number(120.0));
expect_value!(logic, Value::Number(1.0));

expect_error!(error_undefined, Error::IdentifierIsNotDefined(_));