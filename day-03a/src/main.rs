use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let total: i32 = lines
        .map(|line| {
            let line = line.as_ref().unwrap();

            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [left, right])| {
                    left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
                })
                .sum::<i32>()
        })
        .sum();

    println!("{}", total);
}
