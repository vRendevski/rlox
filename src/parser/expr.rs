use super::value::Value;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
  Unary {
    op: Token,
    right: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
  },
  Grouping(Box<Expr>),
  Literal(Value),
  Variable {
    id: usize,
    variable: Token,
  },
  Assignment {
    id: usize,
    variable: Token,
    expr: Box<Expr>,
  },
  Logical {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
  },
  FunCall {
    callee: Box<Expr>,
    paren: Token,
    args: Vec<Box<Expr>>,
  },
}
