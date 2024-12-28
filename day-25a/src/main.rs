use std::io;

type CharGrid = Vec<Vec<char>>;

fn measure(grid: &CharGrid) -> Vec<usize> {
    let mut result = Vec::new();

    for x in 0..5 {
        let mut length = 0;

        for y in 0..7 {
            if grid[y][x] == '#' {
                length += 1;
            }
        }

        result.push(length - 1)
    }

    result
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let locks_and_keys: Vec<CharGrid> = input
        .split("\n\n")
        .map(|lock_or_key| {
            lock_or_key
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();
    let locks: Vec<_> = locks_and_keys
        .iter()
        .filter(|grid| grid[0][0] == '#')
        .collect();
    let keys: Vec<_> = locks_and_keys
        .iter()
        .filter(|grid| grid[0][0] == '.')
        .collect();

    let mut matches = 0;

    for &lock in &locks {
        for &key in &keys {
            let lock_heights = measure(lock);
            let key_heights = measure(key);
            if lock_heights
                .iter()
                .zip(key_heights)
                .all(|(lock_h, key_h)| lock_h + key_h <= 5)
            {
                matches += 1;
            }
        }
    }

    println!("{}", matches);
}
