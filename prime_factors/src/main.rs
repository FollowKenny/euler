fn main() {
    const NUMBER: u64 = 600_851_475_143;
    let mut number: u64 = NUMBER;
    let mut result: Vec<u64> = Vec::new();

    if number % 2 == 0 {
        number = drain_prime(2, number);
        result.push(1);
    }

    let mut i = 3;
    while (i * i) < NUMBER {
        if number % i == 0 {
            number = drain_prime(i, number);
            result.push(i);
        }
        i += 2;
    }

    println!("les diviseurs premiers de {} sont {:?}", NUMBER, result);
}

fn drain_prime (prime: u64, mut number: u64) -> u64 {
    while number % prime == 0 {
        number /= prime;
    }
    number
}
