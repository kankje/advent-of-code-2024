use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;
type Vec2 = (i32, i32);

fn walk(grid: &CharGrid, pos: Vec2, next: i32) -> i32 {
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let width = grid[0].len();
    let height = grid.len();
    let mut count = 0;

    for dir in dirs {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if next_pos.0 < 0
            || next_pos.0 >= width as i32
            || next_pos.1 < 0
            || next_pos.1 >= height as i32
        {
            continue;
        }
        let next_char = grid[next_pos.1 as usize][next_pos.0 as usize];
        if next_char == next.to_string().chars().next().unwrap() {
            if next == 9 {
                count += 1;
            } else {
                count += walk(grid, next_pos, next + 1);
            }
        }
    }

    count
}

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '0' {
                continue;
            }

            count += walk(&grid, (x as i32, y as i32), 1);
        }
    }

    println!("{}", count);
}
