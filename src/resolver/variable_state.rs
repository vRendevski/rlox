use super::ResolveError;
use crate::lexer::Token;

#[derive(Debug)]
pub struct VariableState {
  token: Token,
  ever_assigned: bool,
  ever_read: bool,
}

impl VariableState {
  pub fn new(token: Token) -> VariableState {
    VariableState {
      token,
      ever_assigned: false,
      ever_read: false,
    }
  }

  pub fn mark_assigned(&mut self) {
    self.ever_assigned = true;
  }

  pub fn mark_read(&mut self) {
    self.ever_read = true;
  }

  pub fn check(self) -> Result<(), ResolveError> {
    if !self.ever_assigned {
      return Err(ResolveError::UnassignedVariable(self.token));
    }
    if !self.ever_read {
      return Err(ResolveError::UnusedVariable(self.token));
    }
    Ok(())
  }
}
