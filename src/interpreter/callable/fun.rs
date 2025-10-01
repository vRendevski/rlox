use crate::interpreter::Interpreter;
use crate::interpreter::environment::Environment;
use crate::interpreter::errors::RuntimeError;
use crate::lexer::{Token, TokenKind};
use crate::parser::{ControlSignal, LoxCallable, Stmt, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxFunction {
  name: Token,
  params: Vec<Token>,
  body: Box<Stmt>,
  environment: Rc<RefCell<Environment>>,
}

impl LoxFunction {
  pub fn new(
    name: Token,
    params: Vec<Token>,
    body: Box<Stmt>,
    environment: Rc<RefCell<Environment>>,
  ) -> LoxFunction {
    LoxFunction {
      name,
      params,
      body,
      environment,
    }
  }

  fn declare_params(&self, interpreter: &mut Interpreter, args: Vec<Value>) {
    for (i, arg) in args.iter().enumerate() {
      let param_name = self.params.get(i).unwrap().extract_identifier();
      interpreter.declare(param_name.clone(), arg.clone());
    }
  }
}

impl LoxCallable for LoxFunction {
  fn name(&self) -> &str {
    match self.name.kind() {
      TokenKind::Identifier(iden) => iden,
      _ => panic!("expected identifier for function name"),
    }
  }

  fn arity(&self) -> usize {
    self.params.len()
  }

  fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    assert_eq!(self.params.len(), args.len());
    let old = interpreter.swap_environment(Rc::clone(&self.environment));
    interpreter.begin_scope();
    self.declare_params(interpreter, args);

    let value = match interpreter.eval_stmt(&self.body)? {
      ControlSignal::Return(value) => value,
      _ => Value::Nil,
    };

    interpreter.end_scope();
    interpreter.swap_environment(old);
    Ok(value)
  }
}
