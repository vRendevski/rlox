use std::fmt;

#[derive(Debug)]
pub struct LexerError {
  line: usize,
  col: usize,
  kind: LexerErrorKind,
}

impl LexerError {
  pub fn new(line: usize, col: usize, kind: LexerErrorKind) -> LexerError {
    LexerError { line, col, kind }
  }
}

impl fmt::Display for LexerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "main.lox:l{}:c{} {}", self.line, self.col, self.kind)
  }
}

#[derive(Debug)]
pub enum LexerErrorKind {
  UnexpectedChar(char),
  UnterminatedString,
  InvalidNumber(String),
}

impl fmt::Display for LexerErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedChar(ch) => write!(f, "unexpected char '{}'", ch),
      Self::UnterminatedString => write!(f, "unterminated string"),
      Self::InvalidNumber(n) => write!(f, "invalid number '{}'", n),
    }
  }
}
