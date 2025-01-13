use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Proposition(Token),
}

fn parse_prop(tokens: &[Token]) -> Result<Option<Expr>, ()> {
    Ok(Some(Expr::Proposition(tokens.first().unwrap().clone())))
}

fn parse_unary(tokens: &[Token]) -> Result<Option<Expr>, ()> {
    if let Some(op) = tokens.first() {
        if op.kind == TokenKind::Not {
            let unary = parse_unary(&tokens[1..]).unwrap().unwrap();
            return Ok(Some(Expr::Unary(op.clone(), Box::new(unary))));
        } else {
            return parse_prop(tokens);
        }
    }
    println!("ASDSAD");
    todo!()
}

fn parse_binary(tokens: &[Token]) -> Result<Option<Expr>, ()> {
    let mut expr = parse_unary(tokens);

    let mut i = 1;
    while let Some(op) = tokens.get(i) {
        i += 1;
        if TokenKind::And == op.kind {
            let rhs = parse_unary(&tokens[i..]).unwrap().unwrap();
            expr = Ok(Some(Expr::Binary(
                Box::new(expr.unwrap().unwrap()),
                op.clone(),
                Box::new(rhs),
            )))
        }
    }

    expr
}

fn parse_expr(tokens: &[Token]) -> Result<Option<Expr>, ()> {
    parse_binary(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    parse_expr(&tokens).unwrap().unwrap()
}
