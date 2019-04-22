fn main() {
    const HIGH_DIV: usize = 20;
    let mut result: usize = 1;

    let mut div_vector: Vec<usize> = Vec::new();

    for mut i in 2..=HIGH_DIV {
        if check_prime(i) {
            while i * i <= HIGH_DIV {
                i *= i
            }
            div_vector.push(i)
        }
    }

    for i in &div_vector {
        result *= i
    }

    println!("le plus petit nombre divisible par tous les nombres de\
        1 à {} est {}", HIGH_DIV, result)
}

fn check_prime(num: usize) -> bool {
    for i in 2..=(num / 2) {
        if num % i == 0 {
            return false
        }
    }
    return true
}