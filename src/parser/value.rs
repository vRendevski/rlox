use std::fmt;
use std::rc::Rc;

mod callable;

pub use callable::*;

#[derive(Debug, Clone)]
pub enum Value {
  Number(f64),
  Str(String),
  Bool(bool),
  Nil,
  Callable(Rc<dyn LoxCallable>),
}

impl Value {
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Bool(b) => *b,
      Value::Number(n) => *n != 0.0,
      Value::Str(s) => s.len() > 0,
      Value::Nil => false,
      Value::Callable(_) => true,
    }
  }

  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Number(n) => write!(f, "{n}"),
      Value::Str(s) => write!(f, "{s}"),
      Value::Bool(b) => write!(f, "{b}"),
      Value::Nil => write!(f, "nil"),
      Value::Callable(rc) => write!(f, "<callable {}>", rc.name()),
    }
  }
}
