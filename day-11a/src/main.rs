use std::io;
use std::io::BufRead;

fn blink(stones: Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| {
            if *stone == 0 {
                return vec![1];
            }

            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);
                let left = left.parse::<u64>().unwrap_or(0);
                let right = right.parse::<u64>().unwrap_or(0);
                return vec![left, right];
            }

            vec![*stone * 2024]
        })
        .collect()
}

fn main() {
    let lines = io::stdin().lock().lines();

    let stones: Vec<u64> = lines
        .flat_map(|line| {
            line.unwrap()
                .split(" ")
                .filter_map(|stone| stone.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect();

    let blinked_stones = (0..25).fold(stones, |result, _| blink(result));

    println!("{}", blinked_stones.len());
}
