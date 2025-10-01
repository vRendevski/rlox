use core::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct ParseError {
  token: Token,
  kind: ParseErrorKind,
}

impl ParseError {
  pub fn at(token: Token, kind: ParseErrorKind) -> ParseError {
    ParseError { token, kind }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}, {}", self.token, self.kind)
  }
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
  ExpectedExpression,
  Expected(&'static str),
}

impl fmt::Display for ParseErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ExpectedExpression => write!(f, "expected an expression"),
      Self::Expected(str) => write!(f, "expected '{}'", str),
    }
  }
}
