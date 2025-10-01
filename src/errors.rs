use std::error::Error;
use std::fmt;

use crate::{
  interpreter::RuntimeError, lexer::LexerError, parser::ParseError, resolver::ResolveError,
};

#[derive(Debug)]
pub enum LoxError {
  IoError(Box<dyn Error>),
  LexerError(LexerError),
  ParseError(ParseError),
  ResolveError(ResolveError),
  RuntimeError(RuntimeError),
}

impl fmt::Display for LoxError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::IoError(err) => write!(f, "{}", err),
      Self::LexerError(err) => write!(f, "LexerError: {}", err),
      Self::ParseError(err) => write!(f, "ParseError: {}", err),
      Self::ResolveError(err) => write!(f, "ResolveError: {}", err),
      Self::RuntimeError(err) => write!(f, "RuntimeError: {}", err),
    }
  }
}
