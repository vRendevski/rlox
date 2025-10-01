use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
  line: usize,
  col: usize,
  kind: TokenKind,
}

impl Token {
  pub fn new(line: usize, col: usize, kind: TokenKind) -> Token {
    Token { line, col, kind }
  }

  pub fn kind(&self) -> &TokenKind {
    &self.kind
  }

  pub fn extract_identifier(&self) -> &String {
    match self.kind() {
      TokenKind::Identifier(name) => name,
      _ => panic!("expected identifier token"),
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "line {} col {} token {}", self.line, self.col, self.kind)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  Identifier(String),
  String(String),
  Number(f64),

  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
}

impl TokenKind {
  pub fn name(&self) -> &'static str {
    match self {
      TokenKind::LeftParen => "(",
      TokenKind::RightParen => ")",
      TokenKind::LeftBrace => "{",
      TokenKind::RightBrace => "}",
      TokenKind::Comma => ",",
      TokenKind::Dot => ".",
      TokenKind::Minus => "-",
      TokenKind::Plus => "+",
      TokenKind::Semicolon => ";",
      TokenKind::Slash => "/",
      TokenKind::Star => "*",

      TokenKind::Bang => "!",
      TokenKind::BangEqual => "!=",
      TokenKind::Equal => "=",
      TokenKind::EqualEqual => "==",
      TokenKind::Greater => ">",
      TokenKind::GreaterEqual => ">=",
      TokenKind::Less => "<",
      TokenKind::LessEqual => "<=",

      TokenKind::Identifier(_) => "identifier",
      TokenKind::String(_) => "string",
      TokenKind::Number(_) => "number",

      TokenKind::And => "and",
      TokenKind::Class => "class",
      TokenKind::Else => "else",
      TokenKind::False => "false",
      TokenKind::Fun => "function",
      TokenKind::For => "for",
      TokenKind::If => "if",
      TokenKind::Nil => "nil",
      TokenKind::Or => "or",
      TokenKind::Print => "print",
      TokenKind::Return => "return",
      TokenKind::Super => "super",
      TokenKind::This => "this",
      TokenKind::True => "true",
      TokenKind::Var => "var",
      TokenKind::While => "while",

      TokenKind::Eof => "eof",
    }
  }
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Identifier(iden) => write!(f, "identifier '{}'", iden),
      Self::Number(n) => write!(f, "number '{}'", n),
      Self::String(s) => write!(f, "string '{}'", s),
      _ => write!(f, "{}", self.name()),
    }
  }
}
