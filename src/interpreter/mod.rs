use std::cell::RefCell;
use std::io::{Write, stdout};
use std::rc::Rc;

use crate::lexer::{Token, TokenKind};
use crate::parser::{ControlSignal, Expr, Stmt, Value};
use crate::resolver::Resolver;

mod callable;
mod environment;
mod errors;

use callable::*;
use environment::*;
pub use errors::*;

pub struct Interpreter<'a> {
  environment: Rc<RefCell<Environment>>,
  resolver: Resolver,
  out: Box<dyn Write + 'a>,
}

impl<'a> Interpreter<'a> {
  pub fn new(resolver: Resolver) -> Self {
    Interpreter {
      environment: Rc::new(RefCell::new(Environment::new())),
      resolver,
      out: Box::new(stdout()),
    }
  }

  pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
    for stmt in &stmts {
      self.eval_stmt(stmt)?;
    }
    Ok(())
  }

  pub fn set_out_writer(&mut self, out: Box<dyn Write + 'a>) {
    self.out = out;
  }

  pub fn begin_scope(&mut self) {
    let block = Rc::new(RefCell::new(Environment::with_enclosing(Rc::clone(
      &self.environment,
    ))));
    self.environment = block;
  }

  pub fn end_scope(&mut self) {
    let block = self.environment.borrow().enclosing();
    self.environment = block;
  }

  pub fn swap_environment(&mut self, other: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
    let old = Rc::clone(&self.environment);
    self.environment = other;
    old
  }

  pub fn declare(&mut self, name: String, value: Value) {
    self.environment.borrow_mut().declare(name, value);
  }

  pub fn get(&self, id: usize, name: &String) -> Value {
    let depth = self.resolver.get_bound_depth(id);
    self.environment.borrow().get_at_depth(depth, name)
  }

  pub fn assign(&mut self, id: usize, name: &String, value: &Value) {
    let depth = self.resolver.get_bound_depth(id);
    self
      .environment
      .borrow_mut()
      .assign_at_depth(depth, name, value);
  }

  pub fn eval_expr(&mut self, expr: &Box<Expr>) -> Result<Value, RuntimeError> {
    let value = match &**expr {
      Expr::Unary { op, right } => self.eval_unary_expr(op, right)?,
      Expr::Binary { left, op, right } => self.eval_binary_expr(left, op, right)?,
      Expr::Grouping(expr) => self.eval_expr(expr)?,
      Expr::Literal(value) => value.clone(),
      Expr::Variable { id, variable } => self.eval_variable(id, variable)?,
      Expr::Assignment { id, variable, expr } => self.eval_assignment(id, variable, expr)?,
      Expr::Logical { left, op, right } => self.eval_logical_expr(left, op, right)?,
      Expr::FunCall {
        callee,
        paren,
        args,
      } => self.eval_fun_call(callee, paren, args)?,
    };

    Ok(value)
  }

  fn eval_unary_expr(&mut self, op: &Token, right: &Box<Expr>) -> Result<Value, RuntimeError> {
    let value = self.eval_expr(right)?;
    let result = match op.kind() {
      TokenKind::Minus => match value {
        Value::Number(n) => Value::Number(-n),
        _ => return Err(RuntimeError::ExpectedNumber(op.clone())),
      },
      TokenKind::Bang => Value::Bool(value.is_falsy()),
      _ => panic!("eval unary node with non-unary token"),
    };

    Ok(result)
  }

  fn eval_binary_expr(
    &mut self,
    left: &Box<Expr>,
    op: &Token,
    right: &Box<Expr>,
  ) -> Result<Value, RuntimeError> {
    let left_val = self.eval_expr(left)?;
    let right_val = self.eval_expr(right)?;
    let result = match op.kind() {
      TokenKind::Star => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },
      TokenKind::Slash => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },

      TokenKind::Plus => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
        (Value::Str(a), Value::Str(b)) => Value::Str(a.clone() + b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },
      TokenKind::Minus => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },

      TokenKind::Greater => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a > b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a > b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },
      TokenKind::GreaterEqual => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a >= b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a >= b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },
      TokenKind::Less => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a < b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a < b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },
      TokenKind::LessEqual => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a <= b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a <= b),
        _ => {
          return Err(RuntimeError::UndefinedOpBetween(
            left_val,
            op.clone(),
            right_val,
          ));
        }
      },

      TokenKind::EqualEqual => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a == b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a == b),
        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
        (Value::Nil, Value::Nil) => Value::Bool(true),
        _ => Value::Bool(false),
      },
      TokenKind::BangEqual => match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => Value::Bool(a != b),
        (Value::Str(a), Value::Str(b)) => Value::Bool(a != b),
        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
        (Value::Nil, Value::Nil) => Value::Bool(false),
        _ => Value::Bool(true),
      },

      _ => panic!("binary node received non-binary token"),
    };

    Ok(result)
  }

  fn eval_variable(&self, id: &usize, variable: &Token) -> Result<Value, RuntimeError> {
    let name = variable.extract_identifier();
    let value = self.get(*id, name);
    Ok(value)
  }

  fn eval_assignment(
    &mut self,
    id: &usize,
    variable: &Token,
    expr: &Box<Expr>,
  ) -> Result<Value, RuntimeError> {
    let value = self.eval_expr(expr)?;
    let name = variable.extract_identifier();
    self.assign(*id, name, &value);

    Ok(value)
  }

  fn eval_logical_expr(
    &mut self,
    left: &Box<Expr>,
    op: &Token,
    right: &Box<Expr>,
  ) -> Result<Value, RuntimeError> {
    let value = match op.kind() {
      TokenKind::Or => {
        let left_val = self.eval_expr(left)?;
        if left_val.is_truthy() {
          Value::Bool(true)
        } else {
          let right_val = self.eval_expr(right)?;
          Value::Bool(right_val.is_truthy())
        }
      }
      TokenKind::And => {
        let left_val = self.eval_expr(left)?;
        if left_val.is_falsy() {
          Value::Bool(false)
        } else {
          let right_val = self.eval_expr(right)?;
          Value::Bool(right_val.is_truthy())
        }
      }
      _ => panic!("logical node received non-logical token"),
    };

    Ok(value)
  }

  fn eval_fun_call(
    &mut self,
    callee: &Box<Expr>,
    paren: &Token,
    args: &Vec<Box<Expr>>,
  ) -> Result<Value, RuntimeError> {
    let value = self.eval_expr(callee)?;
    if let Value::Callable(callable) = value {
      if callable.arity() != args.len() {
        return Err(RuntimeError::CallableBadArgsCount(paren.clone()));
      }
      let args: Vec<Value> = args
        .iter()
        .map(|arg| self.eval_expr(arg))
        .collect::<Result<_, _>>()?;
      Ok(callable.call(self, args)?)
    } else {
      Err(RuntimeError::ExpectedCallable(paren.clone()))
    }
  }

  pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<ControlSignal, RuntimeError> {
    match &stmt {
      Stmt::PrintStmt { expr } => self.eval_print_stmt(expr),
      Stmt::ExprStmt { expr } => self.eval_expr_stmt(expr),
      Stmt::VarDecl { variable, expr } => self.eval_var_decl(variable, expr),
      Stmt::Block { stmts } => self.eval_block(stmts),
      Stmt::If {
        condition,
        then_stmt,
        else_stmt,
      } => self.eval_if_stmt(condition, then_stmt, else_stmt),
      Stmt::While { condition, body } => self.eval_while_stmt(condition, body),
      Stmt::FunDecl { name, params, body } => self.eval_fun_decl(name, params, body),
      Stmt::Return { expr } => self.eval_return_stmt(expr),
    }
  }

  fn eval_print_stmt(&mut self, expr: &Box<Expr>) -> Result<ControlSignal, RuntimeError> {
    let value = self.eval_expr(expr)?;
    writeln!(self.out, "{value}").expect("expected that writing to out buffer works");
    Ok(ControlSignal::None)
  }

  fn eval_expr_stmt(&mut self, expr: &Box<Expr>) -> Result<ControlSignal, RuntimeError> {
    self.eval_expr(expr)?;
    Ok(ControlSignal::None)
  }

  fn eval_var_decl(
    &mut self,
    variable: &Token,
    expr: &Option<Box<Expr>>,
  ) -> Result<ControlSignal, RuntimeError> {
    let value = if let Some(expr) = expr {
      self.eval_expr(expr)?
    } else {
      Value::Nil
    };
    let name = variable.extract_identifier().clone();
    self.declare(name, value);
    Ok(ControlSignal::None)
  }

  fn eval_block(&mut self, stmts: &Vec<Stmt>) -> Result<ControlSignal, RuntimeError> {
    self.begin_scope();
    for stmt in stmts {
      let signal = self.eval_stmt(stmt)?;
      let ControlSignal::None = signal else {
        self.end_scope();
        return Ok(signal);
      };
    }
    self.end_scope();
    Ok(ControlSignal::None)
  }

  fn eval_if_stmt(
    &mut self,
    condition: &Box<Expr>,
    then_stmt: &Box<Stmt>,
    else_stmt: &Option<Box<Stmt>>,
  ) -> Result<ControlSignal, RuntimeError> {
    let value = self.eval_expr(condition)?;
    if value.is_truthy() {
      self.eval_stmt(then_stmt)
    } else if let Some(else_stmt) = else_stmt {
      self.eval_stmt(else_stmt)
    } else {
      Ok(ControlSignal::None)
    }
  }

  fn eval_while_stmt(
    &mut self,
    condition: &Box<Expr>,
    body: &Box<Stmt>,
  ) -> Result<ControlSignal, RuntimeError> {
    loop {
      let value = self.eval_expr(condition)?;
      if value.is_truthy() {
        self.eval_stmt(body)?;
      } else {
        break;
      }
    }
    Ok(ControlSignal::None)
  }

  fn eval_fun_decl(
    &mut self,
    name: &Token,
    params: &Vec<Token>,
    body: &Box<Stmt>,
  ) -> Result<ControlSignal, RuntimeError> {
    let value = Value::Callable(Rc::new(LoxFunction::new(
      name.clone(),
      params.clone(),
      body.clone(),
      Rc::clone(&self.environment),
    )));
    self
      .environment
      .borrow_mut()
      .declare(name.extract_identifier().clone(), value);
    Ok(ControlSignal::None)
  }

  fn eval_return_stmt(&mut self, expr: &Box<Expr>) -> Result<ControlSignal, RuntimeError> {
    let value = self.eval_expr(expr)?;
    Ok(ControlSignal::Return(value))
  }
}
