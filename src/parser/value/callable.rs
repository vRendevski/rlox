use super::Value;
use crate::interpreter::{Interpreter, RuntimeError};
use std::fmt;

pub trait LoxCallable: fmt::Debug {
  fn name(&self) -> &str;
  fn arity(&self) -> usize;
  fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError>;
}
