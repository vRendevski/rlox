use std::fmt;

use crate::lexer::Token;
use crate::parser::Value;

#[derive(Debug)]
pub enum RuntimeError {
  UndefinedOpBetween(Value, Token, Value),
  ExpectedNumber(Token),
  CallableBadArgsCount(Token),
  ExpectedCallable(Token),
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UndefinedOpBetween(right, op, left) => {
        write!(
          f,
          "{} is not defined between {:?} and {:?}",
          op, left, right
        )
      }
      Self::ExpectedNumber(tok) => write!(f, "{} expected a number", tok),
      Self::CallableBadArgsCount(tok) => {
        write!(f, "{} called with too few or too many args", tok)
      }
      Self::ExpectedCallable(tok) => {
        write!(f, "{} expected callable", tok)
      }
    }
  }
}
