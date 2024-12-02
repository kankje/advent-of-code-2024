use itertools::Itertools;
use std::io;
use std::io::BufRead;

fn check(levels: &Vec<i32>) -> bool {
    let (is_safe, _) =
        levels
            .iter()
            .tuple_windows()
            .fold((true, 0), |(is_safe, prev_direction), (prev, next)| {
                let new_direction = if next >= prev { 1 } else { -1 };
                let has_direction_changed = prev_direction != 0 && new_direction != prev_direction;
                let diff = (next - prev).abs();
                let is_diff_safe = (1..=3).contains(&diff);
                (
                    is_safe && !has_direction_changed && is_diff_safe,
                    new_direction,
                )
            });

    is_safe
}

fn main() {
    let lines = io::stdin().lock().lines();

    let safe_levels = lines
        .filter(|line| {
            let line = line.as_ref().unwrap();
            let levels: Vec<i32> = line
                .split(" ")
                .map(|part| part.parse::<i32>().unwrap())
                .collect();

            if check(&levels) {
                return true;
            }

            for i in 0..levels.len() {
                let mut without_item: Vec<i32> = levels.clone();
                without_item.remove(i);
                if check(&without_item) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("{}", safe_levels);
}
