fn main() {
    const CEILING: u32 = 4_000_000;

    let result: u32 = fib(1, 2, 0, CEILING);

    println!("la somme des termes pairs de la suite de Fibonacci n'excédant
        pas {} est {}", CEILING, result)
}

fn fib (a: u32, b: u32, mut sum: u32, stopper: u32) -> u32 {
    if b % 2 == 0 {
        sum += b;
    };
    if a + b > stopper {
        sum
    } else {
        fib(b, a + b, sum, stopper)
    }
}
