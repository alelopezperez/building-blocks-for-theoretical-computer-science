use std::collections::{HashMap, HashSet};

use crate::{lexer::TokenKind, parser::Expr};

pub fn optimize(ast: &mut Expr) {}

pub fn generate_env(ast: &Expr) -> Vec<String> {
    let mut set = HashSet::new();

    fn gen_env(ast: &Expr, env: &mut HashSet<String>) {
        match ast {
            Expr::Binary(left_expr, _token, right_expr) => {
                gen_env(left_expr, env);
                gen_env(right_expr, env);
            }
            Expr::Unary(_token, expr) => gen_env(expr, env),
            Expr::Proposition(token) => {
                env.insert(token.lexeme.clone());
            }
            Expr::Grouping(expr) => gen_env(expr, env),
        }
    }
    gen_env(ast, &mut set);
    set.into_iter().collect()
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
        Expr::Unary(token, expr) => {
            let expr = exec_expr(expr, env);
            !expr
        }
        Expr::Proposition(token) => env.iter().find(|v| v.0 == token.lexeme).unwrap().1,
        Expr::Grouping(expr) => exec_expr(expr, env),
    }
}
pub fn truth_table(ast: &Expr) {}
