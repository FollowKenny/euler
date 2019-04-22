use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].clone().parse::<usize>().unwrap();
    let mut num_primes = 0;
    let mut num: usize = 2;

    loop {
        if check_prime(num) {
            num_primes += 1
        }

        if num_primes == n {
            println!("{}", num);
            break
        }

        num += 1
    }
}

fn check_prime(num: usize) -> bool {
    for i in 2..=(num / 2) {
        if num % i == 0 {
            return false
        }
    }
    return true
}