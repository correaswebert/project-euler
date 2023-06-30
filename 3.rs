fn is_prime(n: i64) -> bool {
    let mut i = 3i64;
    let limit = f64::sqrt(n as f64) as i64 + 1;

    while i < limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn main() {
    let n = 600_851_475_143i64;
    let limit = f64::sqrt(n as f64) as i64 + 1;

    let mut i = 3i64;
    let mut factors: Vec<i64> = Vec::new();

    while i < limit {
        if n % i == 0 {
            factors.push(i);
        }
        i += 2;
    }

    for factor in factors.iter().rev() {
        if is_prime(*factor) {
            println!("{}", factor);
            break;
        }
    }
}
