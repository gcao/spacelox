/// A token in the space lox language
#[derive(Debug, Clone)]
pub struct Token {
  /// The token kind
  pub kind: TokenKind,

  /// The character array of the source
  pub lexeme: String,

  /// line number this token appears
  pub line: u32,
}

/// Token kinds in the space lox language
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenKind {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Comma,
  Dot,
  Minus,
  Plus,
  Colon,
  Semicolon,
  Slash,
  Star,

  // logical operators
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // sequences
  Identifier,
  String,
  Number,
  MapOpen,

  // keywords
  And,
  Class,
  Else,
  False,
  For,
  Fun,
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

  // meta
  Error,
  Eof,
}
