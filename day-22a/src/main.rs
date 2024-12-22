use std::io;
use std::io::BufRead;

fn next_secret(secret: u64) -> u64 {
    let mut result = secret;
    result ^= result * 64;
    result %= 16777216;
    result ^= result / 32;
    result %= 16777216;
    result ^= result * 2048;
    result %= 16777216;
    result
}

fn main() {
    let lines = io::stdin().lock().lines();

    let total: u64 = lines
        .map(|line| {
            let line = line.unwrap();
            let mut secret = line.parse::<u64>().unwrap();

            for i in 0..2000 {
                secret = next_secret(secret);
            }

            secret
        })
        .sum();

    println!("{}", total);
}
