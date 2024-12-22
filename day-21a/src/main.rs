use cgmath::Vector2;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::BufRead;

type Pad = Vec<Vec<Option<char>>>;
type Vec2 = Vector2<i32>;

fn dir_to_char(dir: Vec2) -> char {
    match (dir.x, dir.y) {
        (-1, 0) => '<',
        (1, 0) => '>',
        (0, -1) => '^',
        (0, 1) => 'v',
        _ => panic!("Invalid direction"),
    }
}

struct PathState {
    cost: i32,
    pos: Vec2,
    dirs: Vec<Vec2>,
}

fn shortest_paths(pad: &Pad, start: Vec2, end: Vec2) -> Vec<Vec<Vec2>> {
    let width = pad[0].len();
    let height = pad.len();

    let mut frontier = VecDeque::new();

    frontier.push_back(PathState {
        cost: 0,
        pos: start,
        dirs: vec![],
    });

    let mut dirs = Vec::new();
    if end.x < start.x {
        dirs.push(Vec2::new(-1, 0));
    }
    if end.x > start.x {
        dirs.push(Vec2::new(1, 0));
    }
    if end.y < start.y {
        dirs.push(Vec2::new(0, -1));
    }
    if end.y > start.y {
        dirs.push(Vec2::new(0, 1));
    }

    let mut end_states = Vec::new();
    let mut lowest_cost = i32::MAX;

    while let Some(state) = frontier.pop_front() {
        if state.pos == end {
            if lowest_cost < state.cost {
                continue;
            }
            lowest_cost = state.cost;
            end_states.push(state);
            continue;
        }

        if state.pos != start
            && (state.pos.x < 0
                || state.pos.x >= width as i32
                || state.pos.y < 0
                || state.pos.y >= height as i32
                || pad[state.pos.y as usize][state.pos.x as usize].is_none())
        {
            continue;
        }

        for dir in &dirs {
            let mut new_dirs = state.dirs.clone();
            new_dirs.push(*dir);
            frontier.push_back(PathState {
                cost: state.cost + 1,
                pos: Vec2::new(state.pos.x + dir.x, state.pos.y + dir.y),
                dirs: new_dirs,
            });
        }
    }

    end_states.iter().map(|state| state.dirs.clone()).collect()
}

fn lowest_length(
    line: String,
    depth: i32,
    max_depth: i32,
    numpad: &Pad,
    numpad_by_char: &HashMap<char, Vec2>,
    dirpad: &Pad,
    dirpad_by_char: &HashMap<char, Vec2>,
) -> i32 {
    let mut length = 0;

    let mut cursor = if depth == 0 {
        Vec2::new(2, 3)
    } else {
        Vec2::new(2, 0)
    };

    for char in line.chars() {
        let pos = if depth == 0 {
            numpad_by_char[&char]
        } else {
            dirpad_by_char[&char]
        };
        let paths = shortest_paths(if depth == 0 { numpad } else { dirpad }, cursor, pos);
        cursor = pos;

        if depth >= max_depth {
            length += paths.first().unwrap().len() as i32 + 1;
            continue;
        }

        length += paths
            .iter()
            .map(|path| {
                lowest_length(
                    path.iter()
                        .map(|dir| dir_to_char(*dir))
                        .chain(std::iter::once('A'))
                        .collect(),
                    depth + 1,
                    max_depth,
                    numpad,
                    numpad_by_char,
                    dirpad,
                    dirpad_by_char,
                )
            })
            .min()
            .unwrap();
    }

    length
}

fn main() {
    let lines = io::stdin().lock().lines();

    let numpad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];
    let mut numpad_by_char = HashMap::new();
    for y in 0..numpad.len() {
        for x in 0..numpad[0].len() {
            if let Some(c) = numpad[y][x] {
                numpad_by_char.insert(c, Vec2::new(x as i32, y as i32));
            }
        }
    }

    let dirpad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];
    let mut dirpad_by_char = HashMap::new();
    for y in 0..dirpad.len() {
        for x in 0..dirpad[0].len() {
            if let Some(c) = dirpad[y][x] {
                dirpad_by_char.insert(c, Vec2::new(x as i32, y as i32));
            }
        }
    }

    let total: i32 = lines
        .map(|line| {
            let line = line.unwrap();
            lowest_length(
                line.to_string(),
                0,
                2,
                &numpad,
                &numpad_by_char,
                &dirpad,
                &dirpad_by_char,
            ) * line[0..line.len() - 1].parse::<i32>().unwrap()
        })
        .sum();

    println!("{}", total);
}
