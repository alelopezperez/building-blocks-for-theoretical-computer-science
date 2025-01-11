use logical_evluator::{lexer::Lexer, parser::parse};

fn main() {
    let source = "-p";
    let l = Lexer::new(source).collect::<Vec<_>>();
    let p = parse(l);
    println!("{:#?}", p);
}
