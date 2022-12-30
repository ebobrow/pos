use super::token::{Token, TokenType};

use eyre::{bail, Result, WrapErr};

pub struct Lexer {
    src: String,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            pos: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        Ok(match self.advance() {
            Some(c) => match c {
                '+' => Token::new(TokenType::Plus, self.line),
                // TODO: comments `--`
                '-' => Token::new(TokenType::Minus, self.line),
                '*' => Token::new(TokenType::Times, self.line),
                '/' => Token::new(TokenType::Slash, self.line),
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::EqualEqual, self.line)
                    }
                    _ => Token::new(TokenType::Equal, self.line),
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::Geq, self.line)
                    }
                    _ => Token::new(TokenType::Greater, self.line),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::Leq, self.line)
                    }
                    _ => Token::new(TokenType::Less, self.line),
                },
                '(' => Token::new(TokenType::LeftParen, self.line),
                ')' => Token::new(TokenType::RightParen, self.line),
                '.' => Token::new(TokenType::Period, self.line),
                '"' => self.string()?,
                _ => {
                    if c.is_numeric() {
                        self.number()?
                    } else {
                        todo!("kword, ident ({c})")
                    }
                }
            },
            None => Token::new(TokenType::EOF, self.line),
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '\t' | '\r' | ' ' => {
                    self.advance();
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Result<Token> {
        let start = self.pos;
        while let Some(c) = self.advance() {
            if c == '"' {
                return Ok(Token::new(
                    TokenType::String(self.src[start..self.pos - 1].to_string()),
                    self.line,
                ));
            }
        }
        // TODO: nice errors like Rust
        bail!("Unterminated string");
    }

    fn number(&mut self) -> Result<Token> {
        let start = self.pos - 1;
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek() == Some('.') {
            self.advance();
        }
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::new(
            TokenType::Number(self.src[start..self.pos].parse().wrap_err_with(|| {
                format!(
                    "Invalid number on line {}: `{}`",
                    self.line,
                    &self.src[start..self.pos]
                )
            })?),
            self.line,
        ))
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.src.chars().nth(self.pos);
        self.pos += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.pos)
    }
}
