mod errors;
mod tokens;

pub use errors::*;
pub use tokens::*;

#[derive(Debug)]
pub struct Lexer<'a> {
  source_code: &'a str,
  pos: usize,
  line: usize,
  col: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(source_code: &'a str) -> Lexer<'a> {
    Lexer {
      source_code,
      pos: 0,
      line: 1,
      col: 1,
    }
  }

  fn prev(&self) -> Option<char> {
    self.source_code.chars().nth(self.pos - 1)
  }

  fn peek(&self) -> Option<char> {
    self.source_code.chars().nth(self.pos)
  }

  fn consume(&mut self) -> Option<char> {
    if let Some(ch) = self.peek() {
      if ch == '\n' {
        self.line += 1;
        self.col = 0;
      }
      self.pos += 1;
      self.col += 1;

      return Some(ch);
    }

    None
  }

  pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(ch) = self.consume() {
      let token = match ch {
        // Single character tokens
        '(' => TokenKind::LeftParen,
        ')' => TokenKind::RightParen,
        '{' => TokenKind::LeftBrace,
        '}' => TokenKind::RightBrace,
        ',' => TokenKind::Comma,
        '.' => TokenKind::Dot,
        '-' => TokenKind::Minus,
        '+' => TokenKind::Plus,
        ';' => TokenKind::Semicolon,
        '/' => TokenKind::Slash,
        '*' => TokenKind::Star,

        // Potential two character tokens
        '!' => self.match_optional_equal(TokenKind::Bang, TokenKind::BangEqual),
        '=' => self.match_optional_equal(TokenKind::Equal, TokenKind::EqualEqual),
        '>' => self.match_optional_equal(TokenKind::Greater, TokenKind::GreaterEqual),
        '<' => self.match_optional_equal(TokenKind::Less, TokenKind::LessEqual),

        // Strings
        '"' => self.match_string()?,

        // Numbers
        ch if ch.is_ascii_digit() => self.match_number()?,

        // Keywords / Identifiers
        ch if ch.is_ascii_alphabetic() => self.match_identifier_or_keyword(),

        // Whitespace
        ' ' | '\t' | '\r' | '\n' => continue,

        // Invalid
        other => {
          return Err(LexerError::new(
            self.line,
            self.col,
            LexerErrorKind::UnexpectedChar(other),
          ));
        }
      };

      tokens.push(Token::new(self.line, self.col, token));
    }

    tokens.push(Token::new(self.line, self.col, TokenKind::Eof));

    Ok(tokens)
  }

  fn match_optional_equal(&mut self, default: TokenKind, optional: TokenKind) -> TokenKind {
    match self.peek() {
      Some('=') => {
        self.consume();
        optional
      }
      _ => default,
    }
  }

  fn match_string(&mut self) -> Result<TokenKind, LexerError> {
    let mut result = String::new();
    while let Some(ch) = self.peek() {
      if ch == '"' {
        break;
      }
      result.push(ch);
      self.consume();
    }
    if self.peek().is_none() {
      return Err(LexerError::new(
        self.line,
        self.col,
        LexerErrorKind::UnterminatedString,
      ));
    }
    self.consume();
    Ok(TokenKind::String(result))
  }

  fn match_number(&mut self) -> Result<TokenKind, LexerError> {
    let mut result = self.prev().unwrap().to_string();
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_digit() && ch != '.' {
        break;
      }
      result.push(ch);
      self.consume();
    }

    let result: f64 = result.parse().map_err(|_err| {
      LexerError::new(self.line, self.col, LexerErrorKind::InvalidNumber(result))
    })?;

    Ok(TokenKind::Number(result))
  }

  fn match_identifier_or_keyword(&mut self) -> TokenKind {
    let mut value = self.prev().unwrap().to_string();
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_alphanumeric() && ch != '_' {
        break;
      }
      value.push(ch);
      self.consume();
    }

    match value.as_str() {
      "and" => TokenKind::And,
      "class" => TokenKind::Class,
      "else" => TokenKind::Else,
      "false" => TokenKind::False,
      "fun" => TokenKind::Fun,
      "for" => TokenKind::For,
      "if" => TokenKind::If,
      "nil" => TokenKind::Nil,
      "or" => TokenKind::Or,
      "print" => TokenKind::Print,
      "return" => TokenKind::Return,
      "super" => TokenKind::Super,
      "this" => TokenKind::This,
      "true" => TokenKind::True,
      "var" => TokenKind::Var,
      "while" => TokenKind::While,
      _ => TokenKind::Identifier(value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run_lexer(source_code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source_code);
    lexer.tokenize().unwrap()
  }

  #[test]
  fn captures_single_char_tokens() {
    let source_code = "=";
    let tokens = run_lexer(source_code);
    assert_eq!(tokens[0].kind(), &TokenKind::Equal)
  }

  #[test]
  fn captures_two_char_tokens() {
    let source_code = "==";
    let tokens = run_lexer(source_code);
    assert_eq!(tokens[0].kind(), &TokenKind::EqualEqual)
  }

  #[test]
  fn capture_keywors() {
    let source_code = "for";
    let tokens = run_lexer(source_code);
    assert_eq!(tokens[0].kind(), &TokenKind::For)
  }

  #[test]
  fn capture_identifiers() {
    let source_code = "my_identifier";
    let tokens = run_lexer(source_code);
    assert_eq!(
      tokens[0].kind(),
      &TokenKind::Identifier(String::from("my_identifier"))
    )
  }

  #[test]
  fn capture_strings() {
    let source_code = "\"Hello, World!\"";
    let tokens = run_lexer(source_code);
    assert_eq!(
      tokens[0].kind(),
      &TokenKind::String(String::from("Hello, World!"))
    )
  }

  #[test]
  fn capture_numbers() {
    let source_code = "3.14";
    let tokens = run_lexer(source_code);
    assert_eq!(tokens[0].kind(), &TokenKind::Number(3.14))
  }

  #[test]
  #[should_panic]
  fn errors_on_unclosed_str() {
    let source_code = "\"Hello, World!";
    run_lexer(source_code);
  }

  #[test]
  #[should_panic]
  fn errors_on_bad_numbers() {
    let source_code = "3.14.15";
    run_lexer(source_code);
  }
}
