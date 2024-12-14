use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;
type Vec2 = (i32, i32);
type Explored = HashSet<Vec2>;
type Perimeter = HashSet<(Vec2, Vec2)>;

fn explore(grid: &CharGrid, pos: Vec2) -> (Explored, Perimeter) {
    let width = grid[0].len();
    let height = grid.len();

    let plant = grid[pos.1 as usize][pos.0 as usize];

    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut perimeter = HashSet::new();

    queue.push_back(pos);
    explored.insert(pos);

    while let Some(pos) = queue.pop_front() {
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for dir in dirs {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

            if explored.contains(&next_pos) {
                continue;
            }

            if next_pos.0 < 0
                || next_pos.0 >= width as i32
                || next_pos.1 < 0
                || next_pos.1 >= height as i32
                || grid[next_pos.1 as usize][next_pos.0 as usize] != plant
            {
                perimeter.insert((pos, next_pos));
                continue;
            }

            queue.push_back(next_pos);
            explored.insert(next_pos);
        }
    }

    (explored, perimeter)
}

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut all_explored: Explored = HashSet::new();
    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            let pos = (x as i32, y as i32);
            if all_explored.contains(&pos) {
                continue;
            }

            let (explored, perimeter) = explore(&grid, pos);
            let area = explored.len();
            let mut queue: VecDeque<(Vec2, Vec2)> = perimeter.into_iter().collect();
            let mut sides = 0;

            while let Some((inner, outer)) = queue.pop_front() {
                let is_horizontal_side = inner.0 == outer.0;
                let pt = if is_horizontal_side { inner.0 } else { inner.1 };
                let max = if is_horizontal_side { width } else { height } as i32;
                let make_inner = |x_or_y| {
                    if is_horizontal_side {
                        (x_or_y, inner.1)
                    } else {
                        (inner.0, x_or_y)
                    }
                };
                let make_outer = |x_or_y| {
                    if is_horizontal_side {
                        (x_or_y, outer.1)
                    } else {
                        (outer.0, x_or_y)
                    }
                };
                for x_or_y in (0..pt).rev() {
                    let new_inner = make_inner(x_or_y);
                    let new_outer = make_outer(x_or_y);
                    match queue.iter().position(|(i, o)| {
                        i.0 == new_inner.0
                            && i.1 == new_inner.1
                            && o.0 == new_outer.0
                            && o.1 == new_outer.1
                    }) {
                        Some(index) => {
                            queue.remove(index);
                        }
                        None => break,
                    }
                }
                for x_or_y in (pt + 1)..max {
                    let new_inner = make_inner(x_or_y);
                    let new_outer = make_outer(x_or_y);
                    match queue.iter().position(|(i, o)| {
                        i.0 == new_inner.0
                            && i.1 == new_inner.1
                            && o.0 == new_outer.0
                            && o.1 == new_outer.1
                    }) {
                        Some(index) => {
                            queue.remove(index);
                        }
                        None => break,
                    }
                }
                sides += 1;
            }

            total += area * sides;
            all_explored.extend(explored);
        }
    }

    println!("{}", total);
}
