use crate::lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ResolveError {
  UnassignedVariable(Token),
  UnusedVariable(Token),
  UndeclaredVariable(Token),
  OvershadowingSameBlock(Token),
}

impl fmt::Display for ResolveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      ResolveError::UnassignedVariable(tok) => write!(f, "{} unassigned reference", tok),
      ResolveError::UnusedVariable(tok) => write!(f, "{} unused reference", tok),
      ResolveError::UndeclaredVariable(tok) => write!(f, "{} undeclared reference", tok),
      ResolveError::OvershadowingSameBlock(tok) => {
        write!(f, "{} overshadowing reference in the same block", tok)
      }
    }
  }
}
