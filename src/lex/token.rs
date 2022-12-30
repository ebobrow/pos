#[derive(PartialEq, Debug)]
pub enum TokenType {
    Adjective,
    Noun,
    Verb,

    Equal,
    EqualEqual,
    Less,
    Leq,
    Greater,
    Geq,

    Minus,
    Plus,
    Times,
    Slash,

    LeftParen,
    RightParen,
    Period,

    Number(f64),
    String(String),

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    ty: TokenType,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, line: usize) -> Self {
        Self { ty, line }
    }

    pub fn ty(&self) -> &TokenType {
        &self.ty
    }
}
