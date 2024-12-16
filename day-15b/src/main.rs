use cgmath::{MetricSpace, Vector2};
use std::io;

type CharGrid = Vec<Vec<char>>;

fn resolve_movable_boxes(
    grid: &CharGrid,
    pos: (Vector2<i32>, Vector2<i32>),
    dir: Vector2<i32>,
) -> Vec<(Vector2<i32>, Vector2<i32>)> {
    if dir.y == 0 {
        let edge_pos = if dir.x > 0 { pos.1 } else { pos.0 };
        let new_edge_pos = edge_pos + dir;
        let obj_at_new_edge_pos = grid[new_edge_pos.y as usize][new_edge_pos.x as usize];
        if obj_at_new_edge_pos == '#' {
            return vec![];
        }
        if obj_at_new_edge_pos == '.' {
            return vec![pos];
        }
        if obj_at_new_edge_pos == '[' {
            let mut r =
                resolve_movable_boxes(grid, (new_edge_pos, new_edge_pos + Vector2::new(1, 0)), dir);
            if !r.is_empty() {
                r.push(pos);
            }
            return r;
        }
        if obj_at_new_edge_pos == ']' {
            let mut r =
                resolve_movable_boxes(grid, (new_edge_pos - Vector2::new(1, 0), new_edge_pos), dir);
            if !r.is_empty() {
                r.push(pos);
            }
            return r;
        }
        panic!("Invalid object");
    }

    let new_pos = (pos.0 + dir, pos.1 + dir);
    let obj_at_new_pos = (
        grid[new_pos.0.y as usize][new_pos.0.x as usize],
        grid[new_pos.1.y as usize][new_pos.1.x as usize],
    );
    if obj_at_new_pos.0 == '#' || obj_at_new_pos.1 == '#' {
        return vec![];
    }
    if obj_at_new_pos.0 == '.' && obj_at_new_pos.1 == '.' {
        return vec![pos];
    }
    if obj_at_new_pos.0 == '[' && obj_at_new_pos.1 == ']' {
        let mut r = resolve_movable_boxes(grid, new_pos, dir);
        if !r.is_empty() {
            r.push(pos);
        }
        return r;
    }
    if obj_at_new_pos.0 == ']' && obj_at_new_pos.1 == '[' {
        let mut a = resolve_movable_boxes(grid, (new_pos.0 - Vector2::new(1, 0), new_pos.0), dir);
        let b = resolve_movable_boxes(grid, (new_pos.1, new_pos.1 + Vector2::new(1, 0)), dir);
        if !a.is_empty() && !b.is_empty() {
            a.extend(b);
            a.push(pos);
            return a;
        }
        return vec![];
    }
    if obj_at_new_pos.0 == ']' {
        let mut r = resolve_movable_boxes(grid, (new_pos.0 - Vector2::new(1, 0), new_pos.0), dir);
        if !r.is_empty() {
            r.push(pos);
        }
        return r;
    }
    if obj_at_new_pos.1 == '[' {
        let mut r = resolve_movable_boxes(grid, (new_pos.1, new_pos.1 + Vector2::new(1, 0)), dir);
        if !r.is_empty() {
            r.push(pos);
        }
        return r;
    }

    vec![]
}

fn move_box(grid: &mut CharGrid, pos: (Vector2<i32>, Vector2<i32>), dir: Vector2<i32>) -> bool {
    let mut boxes = resolve_movable_boxes(grid, pos, dir);
    if boxes.is_empty() {
        return false;
    }

    boxes.sort_by_key(|(a, _)| a.distance2(pos.0));
    boxes.reverse();

    for (a, b) in boxes {
        let new_a = a + dir;
        let new_b = b + dir;
        grid[a.y as usize][a.x as usize] = '.';
        grid[b.y as usize][b.x as usize] = '.';
        grid[new_a.y as usize][new_a.x as usize] = '[';
        grid[new_b.y as usize][new_b.x as usize] = ']';
    }
    true
}

fn move_obj(grid: &mut CharGrid, pos: Vector2<i32>, dir: Vector2<i32>) -> Vector2<i32> {
    let new_pos = pos + dir;
    let obj_at_new_pos = grid[new_pos.y as usize][new_pos.x as usize];
    if obj_at_new_pos == '#' {
        return pos;
    }
    if obj_at_new_pos == '.' {
        let obj_at_old_pos = grid[pos.y as usize][pos.x as usize];
        grid[new_pos.y as usize][new_pos.x as usize] = obj_at_old_pos;
        grid[pos.y as usize][pos.x as usize] = '.';
        return new_pos;
    }
    if obj_at_new_pos == '[' {
        let was_moved = move_box(grid, (new_pos, new_pos + Vector2::new(1, 0)), dir);
        if was_moved {
            return move_obj(grid, pos, dir);
        }
        return pos;
    }
    if obj_at_new_pos == ']' {
        let was_moved = move_box(grid, (new_pos - Vector2::new(1, 0), new_pos), dir);
        if was_moved {
            return move_obj(grid, pos, dir);
        }
        return pos;
    }
    panic!("Invalid object");
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (grid, instructions) = input.split_once("\n\n").unwrap();

    let mut grid: CharGrid = grid
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!("Invalid char"),
                })
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut robot_position = Vector2::new(0, 0);

    'outer: for x in 0..width {
        for y in 0..height {
            if grid[y][x] == '@' {
                robot_position = Vector2::new(x as i32, y as i32);
                break 'outer;
            }
        }
    }

    for instruction in instructions.chars() {
        robot_position = match instruction {
            '<' => move_obj(&mut grid, robot_position, Vector2::new(-1, 0)),
            '>' => move_obj(&mut grid, robot_position, Vector2::new(1, 0)),
            '^' => move_obj(&mut grid, robot_position, Vector2::new(0, -1)),
            'v' => move_obj(&mut grid, robot_position, Vector2::new(0, 1)),
            _ => continue,
        }
    }

    let mut total = 0;

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == '[' {
                total += y * 100 + x;
            }
        }
    }

    println!("{}", total);
}
