use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].clone().parse::<f32>().unwrap();
    let sum_square = (n + 1.0) * n * (n + (1.0 / 2.0)) / 3.0;
    let square_sum = (n * (n + 1.0) / 2.0).powf(2.0);
    let res = square_sum - sum_square;
    println!("différence {}", res);
}
