use crate::lexer::{Token, TokenKind};

mod control_signal;
mod errors;
mod expr;
mod stmt;
mod value;

pub use control_signal::*;
pub use errors::*;
pub use expr::*;
pub use stmt::*;
pub use value::*;

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  errors: Vec<ParseError>,
  curr_var_id: usize,
}

impl Parser {
  /// # Panics
  ///
  /// The provided token list must always contain
  /// at least one token and end with an Eof.
  pub fn new(tokens: Vec<Token>) -> Parser {
    match tokens.last() {
      Some(token) => match token.kind() {
        TokenKind::Eof => {}
        _ => panic!("token list must end with TokenKind::Eof"),
      },
      None => panic!("token list must not be empty"),
    }
    Parser {
      tokens,
      pos: 0,
      errors: Vec::new(),
      curr_var_id: 0,
    }
  }

  pub fn parse(&mut self) -> Vec<Stmt> {
    let mut stmts: Vec<Stmt> = Vec::new();
    while !self.is_at_end() {
      let decl_res = self.declaration();
      if let Ok(stmt) = decl_res {
        stmts.push(stmt);
      } else if let Err(err) = decl_res {
        self.synchronize(err);
      }
    }
    stmts
  }

  fn incr_var_id(&mut self) -> usize {
    self.curr_var_id = self.curr_var_id + 1;
    self.curr_var_id
  }

  pub fn errors(&self) -> &Vec<ParseError> {
    &self.errors
  }

  fn peek(&self) -> &Token {
    self
      .tokens
      .get(self.pos)
      .expect("expected that consume never lets us go past Eof token")
  }

  fn peek_kind(&self) -> &TokenKind {
    self.peek().kind()
  }

  fn prev(&self, offset: usize) -> Option<&Token> {
    self.tokens.get(self.pos - offset)
  }

  fn is_at_end(&self) -> bool {
    self.peek_kind() == &TokenKind::Eof
  }

  fn consume(&mut self) -> &Token {
    if self.peek_kind() == &TokenKind::Eof {
      return self.peek();
    }
    self.pos += 1;
    self
      .tokens
      .get(self.pos - 1)
      .expect("expected that we never go past Eof token twice")
  }

  fn consume_optional_token<F>(&mut self, matcher: F) -> bool
  where
    F: Fn(&TokenKind) -> bool,
  {
    if matcher(self.peek_kind()) {
      self.consume();
      return true;
    }

    false
  }

  fn consume_optional(&mut self, kind: TokenKind) -> bool {
    self.consume_optional_token(|t| *t == kind)
  }

  fn consume_expect_token<F>(
    &mut self,
    matcher: F,
    expected: &'static str,
  ) -> Result<&Token, ParseError>
  where
    F: Fn(&TokenKind) -> bool,
  {
    if matcher(self.peek_kind()) {
      Ok(self.consume())
    } else {
      Err(ParseError::at(
        self.peek().clone(),
        ParseErrorKind::Expected(expected),
      ))
    }
  }

  fn consume_expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
    self.consume_expect_token(|t| *t == kind, kind.name())
  }

  fn consume_expect_identifier(&mut self) -> Result<Token, ParseError> {
    self
      .consume_expect_token(|t| matches!(t, TokenKind::Identifier(_)), "identifier")
      .map(|t| t.clone())
  }

  fn synchronize(&mut self, error: ParseError) {
    self.errors.push(error);
    loop {
      let token = self.peek();
      match token.kind() {
        TokenKind::Class
        | TokenKind::Fun
        | TokenKind::Var
        | TokenKind::For
        | TokenKind::If
        | TokenKind::While
        | TokenKind::Print
        | TokenKind::Return
        | TokenKind::Eof => return,
        _ => self.consume(),
      };
    }
  }

  fn declaration(&mut self) -> Result<Stmt, ParseError> {
    match self.peek_kind() {
      TokenKind::Var => self.var_decl(),
      TokenKind::Fun => self.fun_decl(),
      _ => self.statement(),
    }
  }

  fn var_decl(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::Var)?;
    let iden = self.consume_expect_identifier()?;
    let mut expr: Option<Expr> = None;
    if self.consume_optional(TokenKind::Equal) {
      expr = Some(self.expression()?);
    }
    self.consume_expect(TokenKind::Semicolon)?;
    Ok(Stmt::VarDecl {
      variable: iden,
      expr: expr.map(|t| Box::new(t)),
    })
  }

  fn fun_decl(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::Fun)?;
    let name = self.consume_expect_identifier()?;
    self.consume_expect(TokenKind::LeftParen)?;
    let params = self.parameters()?;
    self.consume_expect(TokenKind::RightParen)?;
    let body = self.block_stmt()?;

    Ok(Stmt::FunDecl {
      name,
      params,
      body: Box::new(body),
    })
  }

  fn parameters(&mut self) -> Result<Vec<Token>, ParseError> {
    let mut params: Vec<Token> = Vec::new();
    if self.peek_kind() != &TokenKind::RightParen {
      loop {
        params.push(self.consume_expect_identifier()?);
        if self.peek_kind() == &TokenKind::RightParen {
          break;
        }
        self.consume_expect(TokenKind::Comma)?;
      }
    }
    Ok(params)
  }

  fn statement(&mut self) -> Result<Stmt, ParseError> {
    match self.peek_kind() {
      TokenKind::Print => self.print_stmt(),
      TokenKind::LeftBrace => self.block_stmt(),
      TokenKind::If => self.if_stmt(),
      TokenKind::While => self.while_stmt(),
      TokenKind::For => self.for_stmt(),
      TokenKind::Return => self.return_stmt(),
      _ => self.expr_stmt(),
    }
  }

  fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::Print)?;
    let expr = self.expression()?;
    self.consume_expect(TokenKind::Semicolon)?;
    Ok(Stmt::PrintStmt {
      expr: Box::new(expr),
    })
  }

  fn block_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::LeftBrace)?;
    let mut stmts: Vec<Stmt> = Vec::new();
    loop {
      if self.peek_kind() == &TokenKind::RightBrace || self.peek_kind() == &TokenKind::Eof {
        break;
      }
      stmts.push(self.declaration()?);
    }
    self.consume_expect(TokenKind::RightBrace)?;
    Ok(Stmt::Block { stmts })
  }

  fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::If)?;
    self.consume_expect(TokenKind::LeftParen)?;
    let expr = self.expression()?;
    self.consume_expect(TokenKind::RightParen)?;
    let then_stmt = self.statement()?;
    let mut else_stmt: Option<Stmt> = None;
    if self.consume_optional(TokenKind::Else) {
      else_stmt = Some(self.statement()?);
    }
    Ok(Stmt::If {
      condition: Box::new(expr),
      then_stmt: Box::new(then_stmt),
      else_stmt: else_stmt.map(|s| Box::new(s)),
    })
  }

  fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::While)?;
    self.consume_expect(TokenKind::LeftParen)?;
    let expr = self.expression()?;
    self.consume_expect(TokenKind::RightParen)?;
    let body = self.statement()?;
    Ok(Stmt::While {
      condition: Box::new(expr),
      body: Box::new(body),
    })
  }

  fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::For)?;
    self.consume_expect(TokenKind::LeftParen)?;
    let initializer: Option<Stmt>;
    if self.peek_kind() == &TokenKind::Semicolon {
      self.consume();
      initializer = None;
    } else if self.peek_kind() == &TokenKind::Var {
      initializer = Some(self.var_decl()?);
    } else {
      initializer = Some(self.expr_stmt()?);
    }

    let condition: Expr;
    if self.peek_kind() == &TokenKind::Semicolon {
      condition = Expr::Literal(Value::Bool(true));
    } else {
      condition = self.expression()?;
    }

    self.consume_expect(TokenKind::Semicolon)?;

    let increment: Option<Expr>;
    if self.peek_kind() == &TokenKind::RightParen {
      increment = None;
    } else {
      increment = Some(self.expression()?);
    }

    self.consume_expect(TokenKind::RightParen)?;

    let mut body = self.statement()?;

    if let Some(increment) = increment {
      body = Stmt::Block {
        stmts: vec![
          body,
          Stmt::ExprStmt {
            expr: Box::new(increment),
          },
        ],
      }
    }

    let while_stmt = Stmt::While {
      condition: Box::new(condition),
      body: Box::new(body),
    };

    if let Some(initializer) = initializer {
      Ok(Stmt::Block {
        stmts: vec![initializer, while_stmt],
      })
    } else {
      Ok(while_stmt)
    }
  }

  fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume_expect(TokenKind::Return)?;
    let expr = self.expression()?;
    self.consume_expect(TokenKind::Semicolon)?;
    Ok(Stmt::Return {
      expr: Box::new(expr),
    })
  }

  fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
    let expr = self.expression()?;
    self.consume_expect(TokenKind::Semicolon)?;
    Ok(Stmt::ExprStmt {
      expr: Box::new(expr),
    })
  }

  fn expression(&mut self) -> Result<Expr, ParseError> {
    self.assignment()
  }

  fn assignment(&mut self) -> Result<Expr, ParseError> {
    let expr = self.or()?;

    if self.consume_optional(TokenKind::Equal) {
      if let Expr::Variable { id, variable } = expr {
        let expr = self.assignment()?;
        return Ok(Expr::Assignment {
          id,
          variable,
          expr: Box::new(expr),
        });
      } else {
        return Err(ParseError::at(
          self
            .prev(2) // behind Equal
            .expect("expected that previously consumed is not discarded")
            .clone(),
          ParseErrorKind::Expected("identifier"),
        ));
      }
    }

    Ok(expr)
  }

  fn parse_logical(
    &mut self,
    subexpr: fn(&mut Self) -> Result<Expr, ParseError>,
    operators: &[TokenKind],
  ) -> Result<Expr, ParseError> {
    let mut expr = subexpr(self)?;

    loop {
      let token = self.peek();
      if !operators.contains(token.kind()) {
        break;
      }
      let op = self.consume().clone();
      let left = expr;
      let right = subexpr(self)?;
      expr = Expr::Logical {
        left: Box::new(left),
        op,
        right: Box::new(right),
      }
    }

    Ok(expr)
  }

  fn or(&mut self) -> Result<Expr, ParseError> {
    self.parse_logical(Self::and, &[TokenKind::Or])
  }

  fn and(&mut self) -> Result<Expr, ParseError> {
    self.parse_logical(Self::equality, &[TokenKind::And])
  }

  fn parse_binary(
    &mut self,
    subexpr: fn(&mut Self) -> Result<Expr, ParseError>,
    operators: &[TokenKind],
  ) -> Result<Expr, ParseError> {
    let mut expr = subexpr(self)?;

    loop {
      let token = self.peek();
      if !operators.contains(token.kind()) {
        break;
      }
      let op = self.consume().clone();
      let left = expr;
      let right = subexpr(self)?;
      expr = Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
      }
    }

    Ok(expr)
  }

  fn equality(&mut self) -> Result<Expr, ParseError> {
    self.parse_binary(
      Self::comparison,
      &[TokenKind::EqualEqual, TokenKind::BangEqual],
    )
  }

  fn comparison(&mut self) -> Result<Expr, ParseError> {
    self.parse_binary(
      Self::term,
      &[
        TokenKind::Greater,
        TokenKind::GreaterEqual,
        TokenKind::Less,
        TokenKind::LessEqual,
      ],
    )
  }

  fn term(&mut self) -> Result<Expr, ParseError> {
    self.parse_binary(Self::factor, &[TokenKind::Plus, TokenKind::Minus])
  }

  fn factor(&mut self) -> Result<Expr, ParseError> {
    self.parse_binary(Self::unary, &[TokenKind::Star, TokenKind::Slash])
  }

  fn unary(&mut self) -> Result<Expr, ParseError> {
    loop {
      let token = self.peek();
      if token.kind() != &TokenKind::Bang && token.kind() != &TokenKind::Minus {
        break;
      }
      let op = self.consume().clone();
      let right = self.unary()?;
      let expr = Expr::Unary {
        op,
        right: Box::new(right),
      };
      return Ok(expr);
    }

    self.call()
  }

  fn call(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.primary()?;

    if self.peek_kind() == &TokenKind::LeftParen {
      self.consume();
      let args = self.arguments()?;
      let tok = self.consume_expect(TokenKind::RightParen)?.clone();
      expr = Expr::FunCall {
        callee: Box::new(expr),
        paren: tok,
        args,
      }
    }

    Ok(expr)
  }

  fn arguments(&mut self) -> Result<Vec<Box<Expr>>, ParseError> {
    let mut args: Vec<Box<Expr>> = Vec::new();
    if self.peek_kind() != &TokenKind::RightParen {
      loop {
        args.push(Box::new(self.expression()?));
        if self.peek_kind() == &TokenKind::RightParen {
          break;
        }
        self.consume_expect(TokenKind::Comma)?;
      }
    }
    Ok(args)
  }

  fn primary(&mut self) -> Result<Expr, ParseError> {
    let token = self.peek();

    let expr = match token.kind() {
      TokenKind::Number(n) => Expr::Literal(Value::Number(n.clone())),
      TokenKind::String(s) => Expr::Literal(Value::Str(s.clone())),
      TokenKind::True => Expr::Literal(Value::Bool(true)),
      TokenKind::False => Expr::Literal(Value::Bool(false)),
      TokenKind::Nil => Expr::Literal(Value::Nil),
      TokenKind::Identifier(_) => Expr::Variable {
        variable: token.clone(),
        id: self.incr_var_id(),
      },
      TokenKind::LeftParen => {
        self.consume();
        let expr = self.expression()?;
        self.consume_expect(TokenKind::RightParen)?;
        return Ok(Expr::Grouping(Box::new(expr)));
      }
      _ => {
        return Err(ParseError::at(
          token.clone(),
          ParseErrorKind::ExpectedExpression,
        ));
      }
    };

    self.consume();

    Ok(expr)
  }
}
