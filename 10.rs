fn main() {
    let limit = 2_000_000 + 1;
    let mut integers = vec![1; limit];
    integers[0] = 0;
    integers[1] = 0;

    let mut sum_primes: u64 = 2;

    for k in (4..limit).step_by(2) {
        integers[k] = 0;
    }

    for i in (3..limit).step_by(2) {
        if integers[i] == 1 {
            sum_primes += i as u64;
            for k in (i*2..limit).step_by(i) {
                integers[k] = 0;
            }
        }
    }

    println!("{sum_primes}");
}
