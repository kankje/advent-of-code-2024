use std::io;
use std::io::BufRead;
use std::iter::zip;

fn main() {
    let lines = io::stdin().lock().lines();

    let (mut left, mut right): (Vec<String>, Vec<String>) = lines
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split_once("   ").unwrap();
            (parts.0.to_string(), parts.1.to_string())
        })
        .unzip();

    left.sort();
    right.sort();

    let total_difference: i32 = zip(left, right)
        .map(|(l, r)| (l.parse::<i32>().unwrap() - r.parse::<i32>().unwrap()).abs())
        .sum();

    println!("{}", total_difference);
}
