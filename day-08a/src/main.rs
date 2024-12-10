use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;
type Pos = (i32, i32);

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '.' {
                continue;
            }

            let v = map.entry(grid[y][x]).or_default();
            v.push((x as i32, y as i32));
        }
    }

    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (_, positions) in map.iter() {
        for combination in positions.iter().combinations(2) {
            let (a, b) = (combination[0], combination[1]);
            let a_to_b = (b.0 - a.0, b.1 - a.1);
            let antinode_a = (a.0 - a_to_b.0, a.1 - a_to_b.1);
            let antinode_b = (b.0 + a_to_b.0, b.1 + a_to_b.1);
            if antinode_a.0 >= 0
                && antinode_a.0 < width as i32
                && antinode_a.1 >= 0
                && antinode_a.1 < height as i32
            {
                antinodes.insert(antinode_a);
            }
            if antinode_b.0 >= 0
                && antinode_b.0 < width as i32
                && antinode_b.1 >= 0
                && antinode_b.1 < height as i32
            {
                antinodes.insert(antinode_b);
            }
        }
    }

    println!("{}", antinodes.len());
}
