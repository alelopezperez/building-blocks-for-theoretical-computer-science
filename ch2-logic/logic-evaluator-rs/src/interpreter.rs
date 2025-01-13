use crate::{lexer::TokenKind, parser::Expr};

pub fn optimize(ast: &mut Expr) {}

pub fn generate_env(ast: &Expr) -> Vec<(String, bool)> {
    match ast {
        Expr::Binary(expr, _token, expr1) => generate_env(expr)
            .into_iter()
            .chain(generate_env(expr1))
            .collect(),
        Expr::Unary(_token, expr) => generate_env(expr),
        Expr::Proposition(token) => {
            vec![(token.lexeme.clone(), true)]
        }
    }
}

pub fn exec_expr(ast: &Expr) -> bool {
    match ast {
        Expr::Binary(expr, token, expr1) => {
            let lhs = exec_expr(expr);
            let rhs = exec_expr(expr1);
            match token.kind {
                TokenKind::Or => lhs || rhs,
                TokenKind::And => lhs && rhs,
                _ => todo!(),
            }
        }
        Expr::Unary(token, expr) => {
            let expr = exec_expr(expr);
            !expr
        }
        Expr::Proposition(token) => true,
    }
}
pub fn truth_table(ast: &Expr) {}
