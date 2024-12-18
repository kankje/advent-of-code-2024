use cgmath::Vector2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;

type CharGrid = Vec<Vec<char>>;
type Vec2 = Vector2<i32>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: Vec2,
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

fn explore(grid: &CharGrid, start: Vec2, end: Vec2) -> (Vec<Vec2>, i32) {
    let size = grid.len();

    let mut costs = HashMap::new();
    let mut previous = HashMap::new();
    let mut heap = BinaryHeap::new();

    costs.insert(start, 0);
    heap.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == end {
            let mut path = Vec::new();
            let mut trace = pos;
            while trace != start {
                path.push(trace);
                trace = *previous.get(&trace).unwrap();
            }
            path.push(start);
            path.reverse();

            return (path, cost);
        }

        if let Some(&stored_distance) = costs.get(&pos) {
            if cost > stored_distance {
                continue;
            }
        }

        let dirs = [
            Vec2::new(-1, 0),
            Vec2::new(1, 0),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
        ];

        for dir in dirs {
            let new_pos = Vec2::new(pos.x + dir.x, pos.y + dir.y);
            if new_pos.x < 0
                || new_pos.x >= size as i32
                || new_pos.y < 0
                || new_pos.y >= size as i32
                || grid[new_pos.y as usize][new_pos.x as usize] == '#'
            {
                continue;
            }
            let new_cost = cost + 1;
            if new_cost < *costs.get(&new_pos).unwrap_or(&i32::MAX) {
                costs.insert(new_pos, new_cost);
                previous.insert(new_pos, pos);
                heap.push(State {
                    cost: new_cost,
                    pos: new_pos,
                });
            }
        }
    }

    panic!("No path found");
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let size = 70;

    let mut grid: CharGrid = (0..=size)
        .map(|_| (0..=size).map(|_| '.').collect())
        .collect();

    for line in lines[0..1024].iter() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        grid[y as usize][x as usize] = '#';
    }

    let (_, cost) = explore(&grid, Vec2::new(0, 0), Vec2::new(size, size));

    println!("{}", cost);
}
