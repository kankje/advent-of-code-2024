use cgmath::Vector2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;
type Vec2 = Vector2<i32>;

fn turns(dir: Vec2) -> [Vec2; 2] {
    match (dir.x, dir.y) {
        (-1, 0) => [Vec2::new(0, 1), Vec2::new(0, -1)],
        (1, 0) => [Vec2::new(0, 1), Vec2::new(0, -1)],
        (0, -1) => [Vec2::new(1, 0), Vec2::new(-1, 0)],
        (0, 1) => [Vec2::new(1, 0), Vec2::new(-1, 0)],
        _ => panic!("Invalid direction"),
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: Vec2,
    dir: Vec2,
    seats: HashSet<Vec2>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn explore(grid: &CharGrid, start: Vec2, end: Vec2) -> HashSet<Vec2> {
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    let start_dir = Vec2::new(1, 0);
    costs.insert((start, start_dir), 0);
    heap.push(State {
        cost: 0,
        pos: start,
        dir: start_dir,
        seats: HashSet::new(),
    });

    let mut best_cost: Option<i32> = None;
    let mut best_seats = HashSet::new();

    while let Some(State {
        cost,
        pos,
        dir,
        mut seats,
    }) = heap.pop()
    {
        seats.insert(pos);

        if pos == end {
            if let Some(c) = best_cost {
                if cost > c {
                    continue;
                }
            };

            best_cost = Some(cost);
            best_seats.extend(seats);
            continue;
        }

        if let Some(&stored_distance) = costs.get(&(pos, dir)) {
            if cost > stored_distance {
                continue;
            }
        }

        let new_pos = Vec2::new(pos.x + dir.x, pos.y + dir.y);
        if grid[new_pos.y as usize][new_pos.x as usize] != '#' {
            let new_cost = cost + 1;
            if new_cost <= *costs.get(&(new_pos, dir)).unwrap_or(&i32::MAX) {
                costs.insert((new_pos, dir), new_cost);
                heap.push(State {
                    cost: new_cost,
                    pos: new_pos,
                    dir,
                    seats: seats.clone(),
                });
            }
        }

        for new_dir in turns(dir) {
            let new_cost = cost + 1000;
            let new_pos = pos + new_dir;
            if grid[new_pos.y as usize][new_pos.x as usize] == '#' {
                continue;
            }
            if new_cost <= *costs.get(&(pos, new_dir)).unwrap_or(&i32::MAX) {
                costs.insert((pos, new_dir), new_cost);
                heap.push(State {
                    cost: new_cost,
                    pos,
                    dir: new_dir,
                    seats: seats.clone(),
                });
            }
        }
    }

    best_seats
}

fn main() {
    let lines = io::stdin().lock().lines();

    let grid: CharGrid = lines
        .map(|line| line.as_ref().unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut start: Option<Vec2> = None;
    let mut end: Option<Vec2> = None;

    'outer: for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                start = Some(Vec2::new(x as i32, y as i32));
            }
            if grid[y][x] == 'E' {
                end = Some(Vec2::new(x as i32, y as i32));
            }

            if start.is_some() && end.is_some() {
                break 'outer;
            }
        }
    }

    let best_seats = explore(&grid, start.unwrap(), end.unwrap());

    println!("{}", best_seats.len());
}