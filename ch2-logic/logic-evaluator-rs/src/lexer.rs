use std::str::Chars;

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
    pub lexeme: String,
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.source.next() {
            Some(c) => match c {
                '-' => Some(Token {
                    lexeme: '-'.to_string(),
                    kind: TokenKind::Not,
                }),
                '^' => Some(Token {
                    lexeme: '^'.to_string(),
                    kind: TokenKind::And,
                }),
                'v' => Some(Token {
                    lexeme: 'v'.to_string(),
                    kind: TokenKind::Or,
                }),

                '(' => Some(Token {
                    lexeme: 'v'.to_string(),
                    kind: TokenKind::OParen,
                }),
                '>' => Some(Token {
                    lexeme: '>'.to_string(),
                    kind: TokenKind::Implication,
                }),

                c if c.is_ascii() => Some(Token {
                    lexeme: c.to_string(),
                    kind: TokenKind::Proposition,
                }),

                _ => Some(Token {
                    lexeme: "".to_string(),
                    kind: TokenKind::Invalid,
                }),
            },
            None => None,
        }
    }
}
