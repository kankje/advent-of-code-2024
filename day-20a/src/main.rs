use cgmath::{MetricSpace, Vector2};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;
type Vec2 = Vector2<i32>;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: Vec2,
    path: Vec<Vec2>,
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

fn find_path(
    grid: &CharGrid,
    start: Vec2,
    end: Vec2,
    is_cheating: bool,
    max_len: i32,
) -> Option<State> {
    let width = grid[0].len();
    let height = grid.len();

    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        pos: start,
        path: vec![start],
    });

    while let Some(state) = heap.pop() {
        if state.pos == end {
            return Some(state);
        }

        if state.pos != start
            && (state.pos.x < 0
                || state.pos.x >= width as i32
                || state.pos.y < 0
                || state.pos.y >= height as i32
                || (!is_cheating && grid[state.pos.y as usize][state.pos.x as usize] == '#')
                || state.cost >= *costs.get(&state.pos).unwrap_or(&i32::MAX)
                || state.path.len() >= max_len as usize)
        {
            continue;
        }

        costs.insert(state.pos, state.cost);

        let dirs = [
            Vec2::new(-1, 0),
            Vec2::new(1, 0),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
        ];

        for dir in dirs {
            let new_pos = Vec2::new(state.pos.x + dir.x, state.pos.y + dir.y);
            let new_cost = state.cost + 1;
            let mut new_path = state.path.clone();
            new_path.push(new_pos);
            heap.push(State {
                cost: new_cost,
                pos: new_pos,
                path: new_path,
            });
        }
    }

    None
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

    let state_without_cheating =
        find_path(&grid, start.unwrap(), end.unwrap(), false, i32::MAX).unwrap();

    let mut count = 0;

    for (cheat_start_index, cheat_start) in state_without_cheating.path.iter().enumerate() {
        let cheat_ends = state_without_cheating
            .path
            .iter()
            .enumerate()
            .skip(cheat_start_index)
            .filter(|(_, v)| v.distance2(*cheat_start) == 4);

        for (cheat_end_index, cheat_end) in cheat_ends {
            let cheat = find_path(&grid, *cheat_start, *cheat_end, true, 3);
            if let Some(cheat) = cheat {
                let saved = cheat_end_index - cheat_start_index - (cheat.path.len() - 1);
                if saved >= 100 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
