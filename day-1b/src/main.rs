use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let (left, right): (Vec<String>, Vec<String>) = lines
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split_once("   ").unwrap();
            (parts.0.to_string(), parts.1.to_string())
        })
        .unzip();

    let right_counts = right.iter().fold(HashMap::new(), |mut acc, value| {
        let counter = acc.entry(value).or_insert(0);
        *counter += 1;
        acc
    });

    let total_similarity: i32 = left
        .iter()
        .map(|l| l.parse::<i32>().unwrap() * right_counts.get(l).unwrap_or(&0))
        .sum();

    println!("{}", total_similarity);
}
