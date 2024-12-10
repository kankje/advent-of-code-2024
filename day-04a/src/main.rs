use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;

enum NextChar {
    Char(char),
    End,
    Invalid,
}

fn next_char(c: char) -> NextChar {
    match c {
        'X' => NextChar::Char('M'),
        'M' => NextChar::Char('A'),
        'A' => NextChar::Char('S'),
        'S' => NextChar::End,
        _ => NextChar::Invalid,
    }
}

fn walk(grid: &CharGrid, pos: (i32, i32), dir: (i32, i32), next: char) -> i32 {
    let width = grid[0].len();
    let height = grid.len();

    if pos.0 < 0 || pos.0 >= width as i32 || pos.1 < 0 || pos.1 >= height as i32 {
        return 0;
    }

    let c = grid[pos.1 as usize][pos.0 as usize];
    if c != next {
        return 0;
    }

    match next_char(c) {
        NextChar::Char(nc) => walk(grid, (pos.0 + dir.0, pos.1 + dir.1), dir, nc),
        NextChar::End => 1,
        _ => 0,
    }
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
            if grid[y][x] != 'X' {
                continue;
            }

            for x_dir in -1..=1 {
                for y_dir in -1..=1 {
                    count += walk(&grid, (x as i32, y as i32), (x_dir, y_dir), 'X');
                }
            }
        }
    }

    println!("{}", count);
}
