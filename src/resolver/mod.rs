use crate::lexer::Token;
use crate::parser::{Expr, Stmt};
use std::collections::HashMap;

mod errors;
mod variable_state;

pub use errors::*;
pub use variable_state::*;

type ExprId = usize;
type LexicalDepth = usize;

pub struct Resolver {
  scopes: Vec<HashMap<String, VariableState>>,
  bindings: HashMap<ExprId, LexicalDepth>,
  errors: Vec<ResolveError>,
}

impl Resolver {
  pub fn new() -> Self {
    Resolver {
      scopes: vec![HashMap::new()],
      bindings: HashMap::new(),
      errors: Vec::new(),
    }
  }

  pub fn resolve(&mut self, stmts: &Vec<Stmt>) {
    self.begin_scope();
    for stmt in stmts {
      if let Err(err) = self.resolve_stmt(stmt) {
        self.errors.push(err);
        continue;
      }
    }

    if let Err(mut errs) = self.end_scope_extensive() {
      self.errors.append(&mut errs);
    }
  }

  pub fn errors(&self) -> &Vec<ResolveError> {
    &self.errors
  }

  pub fn get_bound_depth(&self, id: usize) -> LexicalDepth {
    self
      .bindings
      .get(&id)
      .copied()
      .expect("expected that reference is resolved")
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  fn pop_last_scope(&mut self) -> HashMap<String, VariableState> {
    self
      .scopes
      .pop()
      .expect("expected a call to begin_scope before pop_last_scope")
  }

  fn end_scope(&mut self) -> Result<(), ResolveError> {
    let scope = self.pop_last_scope();

    for variable in scope.into_values() {
      variable.check()?;
    }
    Ok(())
  }

  fn end_scope_extensive(&mut self) -> Result<(), Vec<ResolveError>> {
    let scope = self.pop_last_scope();

    let mut errors: Vec<ResolveError> = Vec::new();
    for variable in scope.into_values() {
      if let Err(err) = variable.check() {
        errors.push(err);
      }
    }

    if errors.len() == 0 {
      Ok(())
    } else {
      Err(errors)
    }
  }

  fn get_last_scope_mut(&mut self) -> &mut HashMap<String, VariableState> {
    self
      .scopes
      .last_mut()
      .expect("expected that we are inside of at least one scope")
  }

  fn declare_optional_assigned(
    &mut self,
    variable_tok: &Token,
    assigned: bool,
  ) -> Result<(), ResolveError> {
    let name = variable_tok.extract_identifier();
    let last = self.get_last_scope_mut();

    if let None = last.get(name) {
      let mut variable_state = VariableState::new(variable_tok.clone());
      if assigned {
        variable_state.mark_assigned();
      }
      last.insert(name.clone(), variable_state);
      Ok(())
    } else {
      Err(ResolveError::OvershadowingSameBlock(variable_tok.clone()))
    }
  }

  fn declare(&mut self, variable: &Token) -> Result<(), ResolveError> {
    self.declare_optional_assigned(variable, false)
  }

  fn declare_assigned(&mut self, variable: &Token) -> Result<(), ResolveError> {
    self.declare_optional_assigned(variable, true)
  }

  fn assign_curr_scope_non_binding(&mut self, iden: &Token) {
    let name = iden.extract_identifier();
    let last = self.get_last_scope_mut();
    let variable_state = last
      .get_mut(name)
      .expect("expected that non binding variable is in current scope");
    variable_state.mark_assigned();
  }

  fn bind_assign_or_access(
    &mut self,
    id: usize,
    token: &Token,
    should_assign: bool,
  ) -> Result<(), ResolveError> {
    let mut depth = 0;
    let name = token.extract_identifier();
    for scope in self.scopes.iter_mut().rev() {
      if let Some(variable) = scope.get_mut(name) {
        if should_assign {
          variable.mark_assigned();
        } else {
          variable.mark_read();
        }
        self.bindings.insert(id, depth);
        return Ok(());
      }
      depth = depth + 1;
    }

    Err(ResolveError::UndeclaredVariable(token.clone()))
  }

  fn bind_assign(&mut self, id: usize, iden: &Token) -> Result<(), ResolveError> {
    self.bind_assign_or_access(id, iden, true)
  }

  fn bind_access(&mut self, id: usize, iden: &Token) -> Result<(), ResolveError> {
    self.bind_assign_or_access(id, iden, false)
  }

  fn resolve_expr(&mut self, expr: &Box<Expr>) -> Result<(), ResolveError> {
    match &**expr {
      Expr::Unary { op, right } => self.resolve_unary_expr(op, right),
      Expr::Binary { left, op, right } => self.resolve_binary_expr(left, op, right),
      Expr::Grouping(expr) => self.resolve_expr(expr),
      Expr::Literal(_) => Ok(()),
      Expr::Variable { id, variable } => self.resolve_variable(id, variable),
      Expr::Assignment { id, variable, expr } => self.resolve_assignment(id, variable, expr),
      Expr::Logical { left, op, right } => self.resolve_logical_expr(left, op, right),
      Expr::FunCall {
        callee,
        paren,
        args,
      } => self.resolve_fun_call(callee, paren, args),
    }
  }

  fn resolve_unary_expr(&mut self, _op: &Token, right: &Box<Expr>) -> Result<(), ResolveError> {
    self.resolve_expr(right)
  }

  fn resolve_binary_expr(
    &mut self,
    left: &Box<Expr>,
    _op: &Token,
    right: &Box<Expr>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(left)?;
    self.resolve_expr(right)?;
    Ok(())
  }

  fn resolve_variable(&mut self, id: &usize, variable: &Token) -> Result<(), ResolveError> {
    self.bind_access(*id, variable)
  }

  fn resolve_assignment(
    &mut self,
    id: &usize,
    variable: &Token,
    expr: &Box<Expr>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(expr)?;
    self.bind_assign(*id, variable)
  }

  fn resolve_logical_expr(
    &mut self,
    left: &Box<Expr>,
    _op: &Token,
    right: &Box<Expr>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(left)?;
    self.resolve_expr(right)?;
    Ok(())
  }

  fn resolve_fun_call(
    &mut self,
    callee: &Box<Expr>,
    _paren: &Token,
    args: &Vec<Box<Expr>>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(callee)?;
    for arg in args {
      self.resolve_expr(arg)?;
    }
    Ok(())
  }

  fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
    match stmt {
      Stmt::PrintStmt { expr } => self.resolve_print_stmt(expr),
      Stmt::ExprStmt { expr } => self.resolve_expr_stmt(expr),
      Stmt::VarDecl { variable, expr } => self.resolve_var_decl(variable, expr),
      Stmt::Block { stmts } => self.resolve_block_stmt(stmts),
      Stmt::If {
        condition,
        then_stmt,
        else_stmt,
      } => self.resolve_if_stmt(condition, then_stmt, else_stmt),
      Stmt::While { condition, body } => self.resolve_while_stmt(condition, body),
      Stmt::FunDecl { name, params, body } => self.resolve_fun_decl(name, params, body),
      Stmt::Return { expr } => self.resolve_return_stmt(expr),
    }
  }

  fn resolve_print_stmt(&mut self, expr: &Box<Expr>) -> Result<(), ResolveError> {
    self.resolve_expr(expr)
  }

  fn resolve_expr_stmt(&mut self, expr: &Box<Expr>) -> Result<(), ResolveError> {
    self.resolve_expr(expr)
  }

  fn resolve_var_decl(
    &mut self,
    variable: &Token,
    expr: &Option<Box<Expr>>,
  ) -> Result<(), ResolveError> {
    if let Some(expr) = expr {
      self.resolve_expr(expr)?;
    }
    self.declare(variable)?;
    if let Some(_) = expr {
      self.assign_curr_scope_non_binding(variable);
    }
    Ok(())
  }

  fn resolve_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), ResolveError> {
    self.begin_scope();
    for stmt in stmts {
      self.resolve_stmt(stmt)?;
    }
    self.end_scope()
  }

  fn resolve_if_stmt(
    &mut self,
    condition: &Box<Expr>,
    then_stmt: &Box<Stmt>,
    else_stmt: &Option<Box<Stmt>>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(condition)?;
    self.resolve_stmt(then_stmt)?;
    if let Some(else_stmt) = else_stmt {
      self.resolve_stmt(else_stmt)?;
    }
    Ok(())
  }

  fn resolve_while_stmt(
    &mut self,
    condition: &Box<Expr>,
    body: &Box<Stmt>,
  ) -> Result<(), ResolveError> {
    self.resolve_expr(condition)?;
    self.resolve_stmt(body)?;
    Ok(())
  }

  fn resolve_fun_decl(
    &mut self,
    name: &Token,
    params: &Vec<Token>,
    body: &Box<Stmt>,
  ) -> Result<(), ResolveError> {
    self.declare_assigned(name)?;
    self.begin_scope();
    for param in params {
      self.declare_assigned(param)?;
    }
    self.resolve_stmt(body)?;
    self.end_scope()?;
    Ok(())
  }

  fn resolve_return_stmt(&mut self, expr: &Box<Expr>) -> Result<(), ResolveError> {
    self.resolve_expr(expr)
  }
}
