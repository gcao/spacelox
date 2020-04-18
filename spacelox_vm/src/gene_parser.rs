use spacelox_core::io::{StdIo};
use spacelox_core::token::{Token, TokenKind};
use crate::gene_scanner::Scanner;

/// The space lox parser. This struct is responsible for
/// advancing the scanner and checking for specific conditions
pub struct Parser<'a, S: StdIo> {
  /// The current token
  pub current: Token,

  /// The previous token
  pub previous: Token,

  /// Has the parser encountered an error
  pub had_error: bool,

  /// Is the parser in panic mode
  pub panic_mode: bool,

  /// Help reference to the backing scanner
  scanner: Scanner<'a>,

  /// The environments standard io access
  stdio: S,
}

impl<'a, S: StdIo> Parser<'a, S> {
  /// Create a new instance of the parser from a source str
  pub fn new(stdio: S, source: &'a str) -> Self {
    Self {
      scanner: Scanner::new(source),
      stdio,
      had_error: false,
      panic_mode: false,
      previous: Token {
        lexeme: "error".to_string(),
        line: 0,
        kind: TokenKind::Error,
      },
      current: Token {
        lexeme: "error".to_string(),
        line: 0,
        kind: TokenKind::Error,
      },
    }
  }

  /// Does the provided token kind match if so advance the
  /// token index
  pub fn match_kind(&mut self, kind: TokenKind) -> bool {
    if !self.check(kind) {
      return false;
    }
    self.advance();
    true
  }

  /// Does the provided token kind match the current kind
  pub fn check(&self, kind: TokenKind) -> bool {
    self.current.kind == kind
  }

  /// Advance the parser a token forward
  pub fn advance(&mut self) {
    self.previous = self.current.clone();
    loop {
      self.current = self.scanner.scan_token();
      if self.current.kind != TokenKind::Error {
        break;
      }

      self.error_at_current(&self.current.lexeme.to_string())
    }
  }

  /// Consume a token and advance the current token index
  pub fn consume(&mut self, kind: TokenKind, message: &str) {
    if self.current.kind == kind {
      self.advance();
      return;
    }

    self.error_at_current(message)
  }

  /// Indicate an error occurred at he current index
  pub fn error_at_current(&mut self, message: &str) {
    let token = self.current.clone();
    self.error_at(token, message);
  }

  /// Indicate an error occurred at the previous index
  pub fn error(&mut self, message: &str) {
    let token = self.previous.clone();
    self.error_at(token, message);
  }

  /// Print an error to the console for a user to address
  fn error_at(&mut self, token: Token, message: &str) {
    if self.panic_mode {
      return;
    }

    self.panic_mode = true;
    self.stdio.eprint(&format!("[line {}] Error", token.line));

    match token.kind {
      TokenKind::Eof => self.stdio.eprint(" at end"),
      TokenKind::Error => (),
      _ => self.stdio.eprint(&format!(" at {}", token.lexeme)),
    }

    self.stdio.eprintln(&format!(": {}", message));
    self.had_error = true;
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
  None,
  Assignment,
  Or,
  And,
  Equality,
  Comparison,
  Term,
  Factor,
  Unary,
  Call,
  Primary,
}

impl Precedence {
  pub fn higher(&self) -> Precedence {
    match self {
      Precedence::None => Precedence::Assignment,
      Precedence::Assignment => Precedence::Or,
      Precedence::Or => Precedence::And,
      Precedence::And => Precedence::Equality,
      Precedence::Equality => Precedence::Comparison,
      Precedence::Comparison => Precedence::Term,
      Precedence::Term => Precedence::Factor,
      Precedence::Factor => Precedence::Unary,
      Precedence::Unary => Precedence::Call,
      Precedence::Call => Precedence::Primary,
      Precedence::Primary => panic!("Primary is highest precedence"),
    }
  }
}

#[derive(Clone)]
pub enum Act {
  And,
  Binary,
  Call,
  Index,
  List,
  Map,
  Dot,
  Grouping,
  Literal,
  Number,
  Or,
  String,
  Super,
  This,
  Unary,
  Variable,
}

#[cfg(test)]
mod test {
}