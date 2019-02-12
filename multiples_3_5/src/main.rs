use std::collections::HashSet;

fn main() {
    const CEILING: u32 = 1000;
    const SMALL_DIV: u32 = 3;
    const GREAT_DIV: u32 = 5;

    let greater_quotient: u32 = if CEILING % SMALL_DIV == 0 {
        CEILING / SMALL_DIV - 1
    } else {
        CEILING / SMALL_DIV
    };
    let smaller_quotient: u32 = if CEILING % GREAT_DIV == 0 {
        CEILING / GREAT_DIV - 1
    } else {
        CEILING / GREAT_DIV
    };
    let mut result: u32 = 0;

    assert!(smaller_quotient < greater_quotient, "Fucked up your quotients boy");

    let small_div_multiples: HashSet<u32> = compute_multiples(SMALL_DIV,
        greater_quotient);
    let great_div_multiples: HashSet<u32> = compute_multiples(GREAT_DIV,
        smaller_quotient);
    
    for multiple in small_div_multiples.union(&great_div_multiples) {
        result += multiple
    }

    println!("the sum of multiple of {} or {} below {} is {}",
        SMALL_DIV, GREAT_DIV, CEILING, result)
}

fn compute_multiples (div: u32, quotient: u32) -> HashSet<u32> {
    let mut div_multiples: HashSet<u32> = HashSet::new();
    for i in 1..(quotient + 1) {
        div_multiples.insert(div * i);
    }
    div_multiples
}