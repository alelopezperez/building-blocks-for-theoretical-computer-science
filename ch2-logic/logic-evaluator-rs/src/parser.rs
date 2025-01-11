use core::panic;

use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Proposition(Token),
}

fn parse_prop(tokens: &[Token]) -> Expr {
    let prop = tokens.get(0).unwrap().clone();
    if prop.kind == TokenKind::Proposition {
        Expr::Proposition(prop)
    } else {
        panic!()
    }
}

fn parse_unary(tokens: &[Token]) -> Expr {
    let op = tokens.get(0).unwrap().clone();

    if op.kind == TokenKind::Not {
        let expr = parse_prop(&tokens[1..]);
        Expr::Unary(op, Box::new(expr))
    } else {
        parse_prop(tokens)
    }
}

fn parse_binary(tokens: &[Token]) -> Expr {
    let lhs = parse_unary(&tokens[1..]);
    let rhs = parse_unary(&tokens[2..]);
    let op = tokens.get(1).unwrap().clone();

    Expr::Binary(Box::new(lhs), op, Box::new(rhs))
}

fn parse_expr(tokens: &[Token]) -> Expr {
    parse_binary(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    parse_expr(&tokens)
}
