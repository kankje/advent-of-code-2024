use cached::proc_macro::cached;
use std::io;
use std::io::BufRead;

#[cached]
fn blink(stone: u64, count: u64) -> u64 {
    if count == 0 {
        return 1;
    }

    if stone == 0 {
        return blink(1, count - 1);
    }

    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let divisor = 10_u64.pow(digits / 2);
        return blink(stone / divisor, count - 1) + blink(stone % divisor, count - 1);
    }

    blink(stone * 2024, count - 1)
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

    let count: u64 = stones.iter().map(|stone| blink(*stone, 75)).sum();

    println!("{}", count);
}
