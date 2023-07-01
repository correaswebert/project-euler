fn main() {
    let mut sum_of_squares = 0;
    let mut sum = 0;
    
    for n in 1..=100 {
        sum_of_squares += n * n;
        sum += n;
    }

    println!("{}", (sum * sum) - sum_of_squares);
}

