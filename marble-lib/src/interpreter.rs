use std::io::{Read, Write};
use std::sync::{Arc, RwLock};

use crate::builtin;

use crate::value::LazyVal;
use crate::{call, environment::{EnvRef, Environment}, error::Error, expr::{Expr, ExprRef}, fun_val, identifier, unit, value::{BuiltIn, Value, ValueRef}};

pub type ValueResult = Result<ValueRef, Error>;

pub struct Interpreter<I: Read,O: Write> {
    environment: EnvRef,
    expr: ExprRef,
    _input: I,
    output: O,
}

#[derive(Debug)]
enum Instruction {
    Unwrap,
    Swap,
    Call(Arc<RwLock<LazyVal>>, EnvRef),
    Test
}

impl <I: Read,O :Write> Interpreter<I,O> {

    pub fn interpret(&mut self) -> ValueResult {
        let value = self.evaluate(ExprRef::clone(&self.expr))?;
        self.unwrap_lazy(value)
    }

    fn evaluate(&mut self, expr: ExprRef) -> ValueResult {
        match expr.as_ref() {
            Expr::Call(_, _) => {
                Ok(LazyVal::uncomputed(expr.clone(), self.environment.clone()))
            },
            Expr::Identifier(ident) => Ok(self.environment.find(*ident).clone()),
            Expr::Value(v) => Ok(ValueRef::clone(v)),
            Expr::Fn(body) => Ok(Value::Fn(ExprRef::clone(body), EnvRef::clone(&self.environment)).new_ref()),
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

        let mut stack = vec![value];
        let mut instructions = vec![Instruction::Unwrap];

        while let Some(instruction) = instructions.pop() {
            match instruction {
                Instruction::Unwrap => {
                    let value = stack.pop().unwrap();
                    let result = match value.as_ref() {
                        Value::Lazy(rw_lock) => {
                            let access = rw_lock.try_read()
                                .map_err(|_| Error::ValueDependsOnItself)?;

                            match &*access {
                                LazyVal::Computed(val) => {
                                    val.clone()
                                },
                                LazyVal::Uncomputed(expr, env) => {
                                    match expr.as_ref() {
                                        Expr::Call(lhs, rhs) => {
                                            let previous_env = EnvRef::clone(&self.environment);
                                            self.environment = EnvRef::clone(env);

                                            instructions.push(Instruction::Unwrap);
                                            instructions.push(Instruction::Call(rw_lock.clone(), previous_env));
                                            instructions.push(Instruction::Test);
                    
                                            let lhs = self.evaluate(ExprRef::clone(lhs))?;
                                            let rhs = self.evaluate(ExprRef::clone(rhs))?;

                                            stack.push(rhs);
                                            
                                            lhs
                                        },
                                        _ => {
                                            self.evaluate(ExprRef::clone(expr))?
                                        }
                                    }
                                },
                            }
                        },
                        _ => value,
                    };

                    if matches!(result.as_ref(), Value::Lazy(_)) {
                        instructions.push(Instruction::Unwrap);
                    }

                    stack.push(result);
                },
                Instruction::Swap => {
                    let len = stack.len();
                    assert!(len >= 2);
                    stack.swap(len - 1, len - 2);
                },
                Instruction::Test => {
                    if !matches!(stack[stack.len() - 1].as_ref(), Value::Builtin(_)) {
                        continue;
                    }

                    instructions.push(Instruction::Swap);
                    instructions.push(Instruction::Unwrap);
                    instructions.push(Instruction::Swap);
                },
                Instruction::Call(rw_lock, env) => {
                    let mut access = rw_lock.try_write()
                        .map_err(|_| Error::ValueDependsOnItself)?;

                    let lhs = stack.pop().unwrap();
                    let result = match lhs.as_ref() {
                        Value::Fn(expr, env) => self.evaluate_fn(expr.clone(), env.clone(), stack.pop().unwrap()),
                        Value::Builtin(function) => {
                            let rhs = stack.pop().unwrap();
                            self.evaluate_builtin(&function, rhs)
                        },
                        _ => { Err(Error::ValueNotCallable(lhs)) }
                    }?;

                    *access = LazyVal::Computed(result.clone());

                    self.environment = env;

                    stack.push(result);
                },
            }
        }

        assert_eq!(stack.len(), 1);

        Ok(stack.pop().unwrap())
    }

    fn evaluate_builtin(&mut self, function: &BuiltIn, rhs: ValueRef) -> ValueResult {
        match function {
            BuiltIn::Print => {
                match rhs.as_ref() {
                    Value::Number(n) => write!(self.output, "{n}"),
                    Value::String(s) => write!(self.output, "{s}"),
                    Value::Unit => write!(self.output, "Unit"),
                    Value::Lazy(_) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => write!(self.output, "Function"),
                    Value::Builtin(_) => write!(self.output, "Builtin Function"),
                }.map_err(|_| Error::OutputNotWritable)?;
        
                Ok(fun_val!(call!(identifier!(0), unit!())))
            },
            BuiltIn::PrintLn => {
                match rhs.as_ref() {
                    Value::Number(n) => writeln!(self.output, "{n}"),
                    Value::String(s) => writeln!(self.output, "{s}"),
                    Value::Unit => writeln!(self.output, "Unit"),
                    Value::Lazy(_) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => writeln!(self.output, "Function"),
                    Value::Builtin(_) => writeln!(self.output, "Builtin Function"),
                }.map_err(|_| Error::OutputNotWritable)?;
        
                Ok(fun_val!(call!(identifier!(0), unit!())))
            },

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
                }
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
                }
            ),

            BuiltIn::Add => Ok(Value::Builtin(BuiltIn::AddOf(rhs.number_for_operator("Add")?)).new_ref()),
            BuiltIn::Sub => Ok(Value::Builtin(BuiltIn::SubOf(rhs.number_for_operator("Sub")?)).new_ref()),
            BuiltIn::Mul => Ok(Value::Builtin(BuiltIn::MulOf(rhs.number_for_operator("Mul")?)).new_ref()),
            BuiltIn::Div => Ok(Value::Builtin(BuiltIn::DivOf(rhs.number_for_operator("Div")?)).new_ref()),

            BuiltIn::AddOf(lhs) => Ok(Value::Number(lhs + rhs.number_for_operator("Add")?).new_ref()),
            BuiltIn::SubOf(lhs) => Ok(Value::Number(lhs - rhs.number_for_operator("Sub")?).new_ref()),
            BuiltIn::MulOf(lhs) => Ok(Value::Number(lhs * rhs.number_for_operator("Mul")?).new_ref()),
            BuiltIn::DivOf(lhs) => Ok(Value::Number(lhs / rhs.number_for_operator("Div")?).new_ref()),
        }
    }

    pub fn new(expr: ExprRef, input: I, output: O) -> Self {
        Self { 
            environment: Environment::root(), 
            expr,
            _input: input,
            output
        }
    }
}