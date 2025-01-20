use logical_evluator::{
    interpreter::{exec_expr, generate_env},
    lexer::Lexer,
    parser::parse,
};

fn truth_table(values: Vec<String>) -> Vec<Vec<(String, bool)>> {
    todo!()
}

fn main() {
    let source = "g^(-p^g)vr";
    let l = Lexer::new(source).collect::<Vec<_>>();
    let p = parse(l);
    //println!("{:#?}", p);

    let var = generate_env(&p);
    println!("what {:#?}", var);

    (0..2_usize.pow(var.len() as u32)).for_each(|n| {
        let mut bin = n;
        let mut v = Vec::new();
        for i in 0..var.len() {
            bin >>= 1;
            v.push((bin & 0x1) == 1);
        }
        let n_env = var.iter().cloned().zip(v.into_iter()).collect::<Vec<_>>();

        println!("{:?}", n_env);
        println!("{}", exec_expr(&p, &n_env));
    });
}
