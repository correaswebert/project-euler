fn is_palindrome(n: i32) -> bool {
    let mut rev_n = 0;
    let mut temp = n;

    while temp > 0 {
        rev_n = rev_n * 10 + temp % 10;
        temp /= 10;
    }

    n == rev_n
}

fn main() {
    let mut x = 999;
    let mut y = 999;
    let mut max_palindrome = 0;

    while x >= 100 {
        let product = x * y;

        if is_palindrome(product) {
            if product > max_palindrome {
                max_palindrome = product;
            }
        }

        y -= 1;
        if y == 99 {
            x -= 1;
            y = 999;
        }
    }
    
    println!("{}", max_palindrome);
}
