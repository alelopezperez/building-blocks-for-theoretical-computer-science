use crate::{
    lexer::{Token, TokenKind},
    parser::Expr,
};

pub fn generate_env(ast: &Expr) -> Vec<String> {
    let mut set = Vec::new();

    fn gen_env(ast: &Expr, env: &mut Vec<String>) {
        match ast {
            Expr::Binary(left_expr, _token, right_expr) => {
                gen_env(left_expr, env);
                gen_env(right_expr, env);
            }
            Expr::Unary(_token, expr) => gen_env(expr, env),
            Expr::Proposition(token) => {
                if !env.iter().any(|s| *s == token.lexeme) {
                    env.push(token.lexeme.clone());
                }
            }
            Expr::Grouping(expr) => gen_env(expr, env),
            Expr::Implication(_expr, _token, _expr1) => todo!(),
        }
    }
    gen_env(ast, &mut set);
    set
}

pub fn optimizer(ast: &mut Expr) {
    match ast {
        Expr::Binary(expr, _token, expr1) => {
            optimizer(expr);
            optimizer(expr1);
        }
        Expr::Unary(_token, expr) => {
            optimizer(expr);
        }
        Expr::Grouping(expr) => {
            optimizer(expr);
        }

        Expr::Implication(expr, _token, expr1) => {
            let a = Expr::Binary(
                Box::new(Expr::Unary(
                    Token {
                        lexeme: '-'.to_string(),
                        kind: TokenKind::Not,
                    },
                    expr.clone(),
                )),
                Token {
                    lexeme: 'v'.to_string(),
                    kind: TokenKind::Or,
                },
                expr1.clone(),
            );
            *ast = a
        }
        _ => {}
    }
}

pub fn exec_expr(ast: &Expr, env: &Vec<(String, bool)>) -> bool {
    match ast {
        Expr::Binary(expr, token, expr1) => {
            let lhs = exec_expr(expr, env);
            let rhs = exec_expr(expr1, env);
            match token.kind {
                TokenKind::Or => lhs || rhs,
                TokenKind::And => lhs && rhs,
                _ => todo!(),
            }
        }
        Expr::Unary(_token, expr) => {
            let expr = exec_expr(expr, env);
            !expr
        }
        Expr::Proposition(token) => env.iter().find(|v| v.0 == token.lexeme).unwrap().1,
        Expr::Grouping(expr) => exec_expr(expr, env),
        Expr::Implication(_expr, _token, _expr1) => todo!(),
    }
}
