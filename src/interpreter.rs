use std::io::{Read, Write};
use std::ops::Deref;

use crate::builtin;

use crate::error::AnnotatedError;
use crate::token::Token;
use crate::value::LazyVal;
use crate::{
    call,
    environment::{EnvRef, Environment},
    error::Error,
    expr::{Expr, ExprRef},
    fun_val, identifier, unit,
    value::{BuiltIn, Value, ValueRef},
};

pub type ValueResult = Result<ValueRef, AnnotatedError>;

pub struct Interpreter<I: Read, O: Write> {
    environment: EnvRef,
    expr: ExprRef,
    _input: I,
    output: O,
}

impl<I: Read, O: Write> Interpreter<I, O> {
    pub fn interpret(&mut self) -> ValueResult {
        let value = self.evaluate(ExprRef::clone(&self.expr))?;
        self.unwrap_lazy(value)
    }

    fn evaluate(&mut self, expr: ExprRef) -> ValueResult {
        match expr.as_ref().deref() {
            Expr::Call(_, _) => Ok(LazyVal::uncomputed(expr.clone(), self.environment.clone())),
            Expr::Identifier(ident) => Ok(self.environment.find(*ident).clone()),
            Expr::Value(v) => Ok(v.clone()),
            Expr::Fn(body) => {
                Ok(Value::Fn(body.clone(), EnvRef::clone(&self.environment)).new_ref())
            }
        }
    }

    fn evaluate_fn(&mut self, expr: ExprRef, environment: EnvRef, value: ValueRef) -> ValueResult {
        let previous = EnvRef::clone(&self.environment);
        self.environment = Environment::extend(environment, value);
        let return_value = self.evaluate(expr);
        self.environment = previous;

        return_value
    }

    fn unwrap_lazy(&mut self, value: ValueRef) -> ValueResult {
        let rw_lock = match value.as_ref() {
            Value::Lazy(rw_lock) => rw_lock,
            _ => return Ok(value),
        };

        let read = rw_lock
            .try_read()
            .map_err(|_| Error::ValueDependsOnItself.annotate(Token::default()))?;

        let (expr, env) = match read.deref() {
            LazyVal::Uncomputed(annotated_expr, environment) => (annotated_expr, environment),
            LazyVal::Computed(value) => return Ok(value.clone()),
        };

        let previous_env = self.environment.clone();
        self.environment = env.clone();

        let result = match expr.expr() {
            Expr::Call(lhs_expr, rhs_expr) => {
                let lhs = self.evaluate(lhs_expr.clone())?;
                let rhs = self.evaluate(rhs_expr.clone())?;

                let lhs = self.unwrap_lazy(lhs)?;

                match &*lhs {
                    Value::Fn(body, env) => self.evaluate_fn(body.clone(), env.clone(), rhs),
                    Value::Builtin(built_in) => {
                        let rhs = self.unwrap_lazy(rhs)?;
                        self.evaluate_builtin(built_in, rhs)
                            .map_err(|err| err.annotate(expr.token))
                    }
                    _ => Err(Error::ValueNotCallable(lhs).annotate(lhs_expr.token)),
                }
            }
            _ => self.evaluate(expr.clone()),
        }?;

        let result = self.unwrap_lazy(result)?;

        self.environment = previous_env;

        let expr_token = expr.token;

        drop(read);

        let mut write = rw_lock
            .try_write()
            .map_err(|_| Error::ValueDependsOnItself.annotate(expr_token))?;

        *write = LazyVal::Computed(result.clone());

        Ok(result)
    }

    fn evaluate_builtin(&mut self, function: &BuiltIn, rhs: ValueRef) -> Result<ValueRef, Error> {
        match function {
            BuiltIn::Print => {
                match rhs.as_ref() {
                    Value::Number(n) => write!(self.output, "{n}"),
                    Value::String(s) => write!(self.output, "{s}"),
                    Value::Unit => write!(self.output, "Unit"),
                    Value::Lazy(_) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => write!(self.output, "Function"),
                    Value::Builtin(_) => write!(self.output, "Builtin Function"),
                }
                .map_err(|_| Error::OutputNotWritable)?;

                Ok(fun_val!(call!(identifier!(0), unit!())))
            }
            BuiltIn::PrintLn => {
                match rhs.as_ref() {
                    Value::Number(n) => writeln!(self.output, "{n}"),
                    Value::String(s) => writeln!(self.output, "{s}"),
                    Value::Unit => writeln!(self.output, "Unit"),
                    Value::Lazy(_) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => writeln!(self.output, "Function"),
                    Value::Builtin(_) => writeln!(self.output, "Builtin Function"),
                }
                .map_err(|_| Error::OutputNotWritable)?;

                Ok(fun_val!(call!(identifier!(0), unit!())))
            }

            BuiltIn::Is => Ok(Value::Builtin(BuiltIn::IsOf(rhs)).new_ref()),
            BuiltIn::IsNot => Ok(Value::Builtin(BuiltIn::IsNotOf(rhs)).new_ref()),

            BuiltIn::IsOf(lhs) => Ok(
                if match (lhs.as_ref(), rhs.as_ref()) {
                    (Value::Number(l0), Value::Number(r0)) => l0 == r0,
                    (Value::String(l0), Value::String(r0)) => l0 == r0,
                    (Value::Unit, Value::Unit) => true,
                    _ => false,
                } {
                    builtin::TRUE.clone()
                } else {
                    builtin::FALSE.clone()
                },
            ),

            BuiltIn::IsNotOf(lhs) => Ok(
                if match (lhs.as_ref(), rhs.as_ref()) {
                    (Value::Number(l0), Value::Number(r0)) => l0 == r0,
                    (Value::String(l0), Value::String(r0)) => l0 == r0,
                    (Value::Unit, Value::Unit) => true,
                    _ => false,
                } {
                    builtin::FALSE.clone()
                } else {
                    builtin::TRUE.clone()
                },
            ),

            BuiltIn::Add => {
                Ok(Value::Builtin(BuiltIn::AddOf(rhs.number_for_operator("Add")?)).new_ref())
            }
            BuiltIn::Sub => {
                Ok(Value::Builtin(BuiltIn::SubOf(rhs.number_for_operator("Sub")?)).new_ref())
            }
            BuiltIn::Mul => {
                Ok(Value::Builtin(BuiltIn::MulOf(rhs.number_for_operator("Mul")?)).new_ref())
            }
            BuiltIn::Div => {
                Ok(Value::Builtin(BuiltIn::DivOf(rhs.number_for_operator("Div")?)).new_ref())
            }

            BuiltIn::AddOf(lhs) => {
                Ok(Value::Number(lhs + rhs.number_for_operator("Add")?).new_ref())
            }
            BuiltIn::SubOf(lhs) => {
                Ok(Value::Number(lhs - rhs.number_for_operator("Sub")?).new_ref())
            }
            BuiltIn::MulOf(lhs) => {
                Ok(Value::Number(lhs * rhs.number_for_operator("Mul")?).new_ref())
            }
            BuiltIn::DivOf(lhs) => {
                Ok(Value::Number(lhs / rhs.number_for_operator("Div")?).new_ref())
            }
        }
    }

    pub fn new(expr: ExprRef, input: I, output: O) -> Self {
        Self {
            environment: Environment::root(),
            expr,
            _input: input,
            output,
        }
    }
}
