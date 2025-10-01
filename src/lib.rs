use std::io::Write;

mod errors;
use errors::LoxError;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser, resolver::Resolver};

mod interpreter;
mod lexer;
mod parser;
mod resolver;

/// Starts interpreting the given file.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if
/// the contents cannot be interpreted in the lox programming language.
pub fn run_file<'a>(
  path: String,
  out_writer: Option<Box<dyn Write + 'a>>,
) -> Result<(), Vec<LoxError>> {
  match std::fs::read_to_string(path) {
    Err(err) => Err(vec![LoxError::IoError(Box::new(err))]),
    Ok(source_code) => run_source_code(&source_code, out_writer),
  }
}

pub fn run_source_code<'a>(
  source_code: &str,
  out_writer: Option<Box<dyn Write + 'a>>,
) -> Result<(), Vec<LoxError>> {
  let mut lexer = Lexer::new(source_code);
  match lexer.tokenize() {
    Err(err) => Err(vec![LoxError::LexerError(err)]),
    Ok(tokens) => {
      let mut parser = Parser::new(tokens);
      let stmts = parser.parse();
      if parser.errors().len() > 0 {
        return Err(
          parser
            .errors()
            .iter()
            .map(|t| LoxError::ParseError(t.clone()))
            .collect(),
        );
      }
      let mut resolver = Resolver::new();
      resolver.resolve(&stmts);
      if resolver.errors().len() > 0 {
        return Err(
          resolver
            .errors()
            .iter()
            .map(|t| LoxError::ResolveError(t.clone()))
            .collect(),
        );
      }
      let mut interpreter = Interpreter::new(resolver);
      if let Some(out_writer) = out_writer {
        interpreter.set_out_writer(out_writer);
      }
      match interpreter.interpret(stmts) {
        Err(err) => Err(vec![LoxError::RuntimeError(err)]),
        Ok(_) => Ok(()),
      }
    }
  }
}
