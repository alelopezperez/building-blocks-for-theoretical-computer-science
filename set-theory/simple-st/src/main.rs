use simple_st::{natural_to_set, union_op};

fn main() {
    println!("Hello, world!");
    let num = 1;

    let set = natural_to_set(num);
    let set_b = natural_to_set(num);
    println!("{num} = {}", set);
    println!("{num} = {}", set_b);

    let un = union_op(set, set_b);
}
