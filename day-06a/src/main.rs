use std::collections::HashSet;
use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;

fn rotate(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn walk(
    grid: &CharGrid,
    pos: (i32, i32),
    dir: (i32, i32),
    mut visited: HashSet<(i32, i32)>,
) -> i32 {
    let width = grid[0].len();
    let height = grid.len();

    visited.insert(pos);

    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if next_pos.0 < 0 || next_pos.0 >= width as i32 || next_pos.1 < 0 || next_pos.1 >= height as i32
    {
        return visited.len() as i32;
    }

    let c = grid[next_pos.1 as usize][next_pos.0 as usize];
    if c == '#' {
        let dir = rotate(dir);
        return walk(grid, (pos.0 + dir.0, pos.1 + dir.1), dir, visited);
    }

    walk(grid, (pos.0 + dir.0, pos.1 + dir.1), dir, visited)
}

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '^' {
                continue;
            }

            println!(
                "{}",
                walk(&grid, (x as i32, y as i32), (0, -1), HashSet::new())
            );
            return;
        }
    }

    println!("Path not found");
}
