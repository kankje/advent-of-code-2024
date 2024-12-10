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

fn walk(grid: &CharGrid, start_pos: (i32, i32)) -> (bool, HashSet<(i32, i32)>) {
    let width = grid[0].len();
    let height = grid.len();

    let start_dir = (0, -1);
    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    loop {
        if visited.contains(&(pos, dir)) {
            return (true, visited.iter().map(|x| x.0).collect());
        }

        visited.insert((pos, dir));

        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if next_pos.0 < 0
            || next_pos.0 >= width as i32
            || next_pos.1 < 0
            || next_pos.1 >= height as i32
        {
            return (false, visited.iter().map(|x| x.0).collect());
        }

        let c = grid[next_pos.1 as usize][next_pos.0 as usize];
        if c == '#' {
            dir = rotate(dir);
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            // Handle corner
            let c = grid[next_pos.1 as usize][next_pos.0 as usize];
            if c == '#' {
                dir = rotate(dir);
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            } else {
                pos = next_pos;
            }
            continue;
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
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

            let start_pos = (x as i32, y as i32);
            let (_, visited) = walk(&grid, start_pos);
            let mut obstacles = HashSet::new();
            for pos in visited.iter() {
                if *pos == start_pos {
                    continue;
                }

                if obstacles.contains(pos) {
                    continue;
                }

                let mut with_obstacle = grid.clone();
                with_obstacle[pos.1 as usize][pos.0 as usize] = '#';
                let (is_loop, _) = walk(&with_obstacle, (x as i32, y as i32));
                if is_loop {
                    obstacles.insert(pos);
                }
            }

            println!("{}", obstacles.len());
            return;
        }
    }

    println!("Path not found");
}
