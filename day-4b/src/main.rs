use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] != 'A' {
                continue;
            }

            // M . .     S . .
            // . A . and . A .
            // . . S     . . M
            let ltr = (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
                || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M');

            // . . M     . . S
            // . A . and . A .
            // S . .     M . .
            let rtl = (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
                || grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M';

            if ltr && rtl {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
