use logical_evluator::{interpreter::exec_expr, lexer::Lexer, parser::parse};

fn main() {
    let source = "pv-p";
    let l = Lexer::new(source).collect::<Vec<_>>();
    let p = parse(l);
    let e = exec_expr(&p);
    println!("{:#?}", p);
    println!("{:#?}", e)
}
