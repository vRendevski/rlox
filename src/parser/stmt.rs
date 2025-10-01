use super::Expr;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
  ExprStmt {
    expr: Box<Expr>,
  },
  PrintStmt {
    expr: Box<Expr>,
  },
  VarDecl {
    variable: Token,
    expr: Option<Box<Expr>>,
  },
  Block {
    stmts: Vec<Stmt>,
  },
  If {
    condition: Box<Expr>,
    then_stmt: Box<Stmt>,
    else_stmt: Option<Box<Stmt>>,
  },
  While {
    condition: Box<Expr>,
    body: Box<Stmt>,
  },
  FunDecl {
    name: Token,
    params: Vec<Token>,
    body: Box<Stmt>,
  },
  Return {
    expr: Box<Expr>,
  },
}
