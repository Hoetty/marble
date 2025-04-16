use std::fs;
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use rust_embed::Embed;

use crate::source::Source;
use crate::{builtin, evaluate_code};

use crate::error::AnnotatedError;
use crate::{
    call,
    environment::{EnvRef, Environment},
    error::Error,
    expr::{Expr, ExprRef},
    fun_val, identifier, unit,
    value::{BuiltIn, Value, ValueRef},
};

pub type ValueResult = Result<ValueRef, AnnotatedError>;
pub type Input<'a> = Arc<Mutex<Box<dyn Read + Send + 'a>>>;
pub type Output<'a> = Arc<Mutex<Box<dyn Write + Send + 'a>>>;

#[derive(Embed)]
#[folder = "examples/lang"]
#[prefix = "lang/"]
#[include("*.mrbl")]
struct Lang;

pub struct Interpreter<'a> {
    execution_path: PathBuf,
    _input: Input<'a>,
    output: Output<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self, expr: ExprRef) -> ValueResult {
        let value = self.evaluate(expr, Environment::root())?;
        self.unwrap_lazy(value)
    }

    fn evaluate(&mut self, expr: ExprRef, environment: EnvRef) -> ValueResult {
        match expr.as_ref().deref() {
            Expr::Call(lhs, rhs) => {
                Ok(
                    Value::LazyCall(lhs.clone(), rhs.clone(), environment, OnceLock::new())
                        .new_ref(),
                )
            }
            Expr::Identifier(ident) => Ok(environment.find(*ident).clone()),
            Expr::Value(v) => Ok(v.clone()),
            Expr::Fn(body) => Ok(Value::Fn(body.clone(), environment).new_ref()),
        }
    }

    fn evaluate_fn(&mut self, expr: ExprRef, environment: EnvRef, value: ValueRef) -> ValueResult {
        self.evaluate(expr, Environment::extend(environment, value))
    }

    fn unwrap_lazy(&mut self, value: ValueRef) -> ValueResult {
        let Value::LazyCall(lhs_expr, rhs_expr, env, cell) = &*value else {
            return Ok(value);
        };

        if let Some(value) = cell.get() {
            return Ok(value.clone());
        }

        let lhs = self.evaluate(lhs_expr.clone(), env.clone())?;
        let rhs = self.evaluate(rhs_expr.clone(), env.clone())?;

        let lhs = self.unwrap_lazy(lhs)?;

        let value = match &*lhs {
            Value::Fn(body, env) => self.evaluate_fn(body.clone(), env.clone(), rhs),
            Value::Builtin(built_in) => {
                let rhs = self.unwrap_lazy(rhs)?;
                self.evaluate_builtin(built_in, rhs)
                    .map_err(|err| err.annotate(lhs_expr.token))
            }
            _ => Err(Error::ValueNotCallable(lhs).annotate(lhs_expr.token)),
        }?;

        let value = self.unwrap_lazy(value)?;

        Ok(cell.get_or_init(|| value).clone())
    }

    fn evaluate_builtin(&mut self, function: &BuiltIn, rhs: ValueRef) -> Result<ValueRef, Error> {
        match function {
            BuiltIn::Print => {
                let mut output = self.output.lock().unwrap();

                match rhs.as_ref() {
                    Value::Number(n) => write!(output, "{n}"),
                    Value::String(s) => write!(output, "{s}"),
                    Value::Unit => write!(output, "Unit"),
                    Value::LazyCall(_, _, _, _) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => write!(output, "Function"),
                    Value::Builtin(_) => write!(output, "Builtin Function"),
                }
                .map_err(|_| Error::OutputNotWritable)?;

                Ok(fun_val!(call!(identifier!(0), unit!())))
            }
            BuiltIn::PrintLn => {
                let mut output = self.output.lock().unwrap();

                match rhs.as_ref() {
                    Value::Number(n) => writeln!(output, "{n}"),
                    Value::String(s) => writeln!(output, "{s}"),
                    Value::Unit => writeln!(output, "Unit"),
                    Value::LazyCall(_, _, _, _) => panic!("Lazy passed to print"),
                    Value::Fn(_, _) => writeln!(output, "Function"),
                    Value::Builtin(_) => writeln!(output, "Builtin Function"),
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
            BuiltIn::Import => match rhs.as_ref() {
                Value::String(source) => self.import(source.clone()),
                _ => Err(Error::ArgumentToImportMustBeAString),
            },
        }
    }

    pub fn import(&mut self, source_file: String) -> Result<ValueRef, Error> {
        if let Some(file) = Lang::get(&format!("{source_file}.mrbl")) {
            let code = std::str::from_utf8(&file.data).unwrap();
            let path = PathBuf::from(&source_file).parent().unwrap().to_path_buf();

            return self.evaluate_imported(code, source_file, path);
        }

        let mut file = self.execution_path.clone();
        file.push(&source_file);
        file.set_extension("mrbl");

        let Ok(code) = fs::read_to_string(file.clone()) else {
            return Err(Error::ImportCouldNotBeResolved(source_file));
        };

        self.evaluate_imported(&code, source_file, file.parent().unwrap().to_path_buf())
    }

    fn evaluate_imported(
        &mut self,
        code: &str,
        source_file: String,
        file_path: PathBuf,
    ) -> Result<ValueRef, Error> {
        let source = Source::new(code);

        evaluate_code(code, self._input.clone(), self.output.clone(), file_path).map_err(
            move |err| {
                let err_string = err.of_source(&source);

                Error::ErrorInImportedFile(source_file, err_string)
            },
        )
    }

    pub fn new(input: Input<'a>, output: Output<'a>, path: PathBuf) -> Self {
        Self {
            _input: input,
            output,
            execution_path: path,
        }
    }
}
