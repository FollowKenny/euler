fn main() {
    const MIN_DIGITS: u32 = 100;
    const MAX_DIGITS: u32 = 999;
    const MAX_NUM: u32 = MAX_DIGITS * MAX_DIGITS;
    let mut result: u32 = 0;
    
    for i in (0..MAX_NUM).rev() {
        if is_palindrome(i) && is_product(i, MIN_DIGITS, MAX_DIGITS) {
            result = i;
            break
        }
    }
    println!("The largest palindrome made from product
        of two 3-digits number is {}", result);
}

fn is_palindrome(n: u32) -> bool {
    let s: String = n.to_string();
    let mut s_rev = s.clone();
    unsafe {
        let vec = s_rev.as_mut_vec();
        vec.reverse();
    }
    s == s_rev
}

fn is_product (n: u32, min: u32, max: u32) -> bool {
    for i in min..max {
        if n % i == 0 {
            if n / i > min && n / i < max {
                return true
            }
        }
    }
    false
}