use logical_evluator::{
    interpreter::{exec_expr, generate_env, optimizer},
    lexer::Lexer,
    parser::parse,
};

fn truth_table(values: Vec<String>) -> Vec<Vec<(String, bool)>> {
    todo!()
}

fn main() {
    let source = "r^(a>b)";
    let l = Lexer::new(source).collect::<Vec<_>>();
    let mut p = parse(l);

    optimizer(&mut p);

    let mut var = generate_env(&p);
    for prop in &var {
        print!("{}        ", prop);
    }
    print!("{source}");

    (0..2_usize.pow(var.len() as u32)).for_each(|n| {
        let mut bin = n;
        let mut v = Vec::new();
        for _i in 0..var.len() {
            v.push((bin & 0x1) == 1);
            bin >>= 1;
        }
        let n_env = var.iter().cloned().zip(v).collect::<Vec<_>>();

        println!("\n-------------------------------------------");
        for prop in &n_env {
            print!("{}     ", prop.1);
        }
        println!("{}", exec_expr(&p, &n_env));
    });
}
