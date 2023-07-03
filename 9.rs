fn main() {
    let mut a = 998;
    let mut b = 1;
    let mut c = 1;

    while a > 0 {
        c = 1000 - a - b;

        if a*a + b*b == c*c {
            break;
        }

        if a + b == 1000 {
            a -= 1;
            b = 1;
        }

        b += 1;
    }

    println!("{}", a * b * c);
}

