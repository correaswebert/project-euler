fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 {
        return y;
    }

    gcd(y % x, x)
}

fn main() {
    let mut lcm = 1;

    // algorithm for LCM(1, 2, ..., 20)
    for n in 1..20 {
        lcm = (lcm * n) / gcd(lcm, n);
    }
    
    println!("{lcm}");
}

