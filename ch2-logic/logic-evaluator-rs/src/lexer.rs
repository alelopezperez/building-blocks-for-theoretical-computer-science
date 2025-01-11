use std::{ascii::AsciiExt, str::Chars};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    And,
    Or,
    Not,
    Implication,
    Proposition,
    OParen,
    CParen,
    EOF,
    Invalid,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Chars<'a>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        println!("{:?}", self.source);
        match self.source.next() {
            Some(c) => match c {
                '-' => Some(Token {
                    kind: TokenKind::Not,
                }),
                '^' => Some(Token {
                    kind: TokenKind::And,
                }),
                'v' => Some(Token {
                    kind: TokenKind::Or,
                }),

                c if c.is_ascii() => Some(Token {
                    kind: TokenKind::Proposition,
                }),

                _ => Some(Token {
                    kind: TokenKind::Invalid,
                }),
            },
            None => None,
        }
    }
}
