use std::iter::Peekable;

use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Proposition(Token),
}

fn parse_prop<'a, I: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<I>,
) -> Result<Option<Expr>, ()> {
    if let Some(prop) = tokens.next() {
        println!("{:?}", prop);
        if let TokenKind::Proposition = prop.kind {
            return Ok(Some(Expr::Proposition(prop.clone())));
        } else if let TokenKind::OParen = prop.kind {
            let expr = parse_expr(tokens).unwrap().unwrap();
            return Ok(Some(Expr::Grouping(Box::new(expr))));
        } else {
            todo!()
        }
    }
    todo!()
}

fn parse_unary<'a, I: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<I>,
) -> Result<Option<Expr>, ()> {
    if let Some(op) = tokens.peek() {
        if let TokenKind::Not = op.kind {
            let op = tokens.next().unwrap();
            let unary = parse_unary(tokens).unwrap().unwrap();
            return Ok(Some(Expr::Unary(op.clone(), Box::new(unary))));
        } else {
            return parse_prop(tokens);
        }
    }
    todo!()
}

fn parse_binary<'a, I: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<I>,
) -> Result<Option<Expr>, ()> {
    let mut expr = parse_unary(tokens);
    while let Some(op) = tokens.next() {
        if TokenKind::And == op.kind || TokenKind::Or == op.kind {
            let rhs = parse_unary(tokens).unwrap().unwrap();
            expr = Ok(Some(Expr::Binary(
                Box::new(expr.unwrap().unwrap()),
                op.clone(),
                Box::new(rhs),
            )))
        }
    }

    expr
}

fn parse_expr<'a, I: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<I>,
) -> Result<Option<Expr>, ()> {
    parse_binary(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    parse_expr(&mut tokens.iter().peekable()).unwrap().unwrap()
}
