fn is_prime(n: u64) -> bool {
    let mut i = 3u64;
    let limit = f64::sqrt(n as f64) as u64 + 1;

    while i < limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}
fn main() {
    // already considered 2 as prime
    let mut prime_count = 1;
    let mut n = 3;

    while prime_count < 10001 {
        if is_prime(n) {
            prime_count += 1;
        }
        n += 2;
    }

    // subtract two as n += 2 done for the 10001st prime
    println!("{}", n-2);
}

