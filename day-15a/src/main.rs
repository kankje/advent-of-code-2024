use cgmath::Vector2;
use std::io;

type CharGrid = Vec<Vec<char>>;

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
    if obj_at_new_pos == 'O' {
        let new_obj_pos = move_obj(grid, new_pos, dir);
        if new_obj_pos != new_pos {
            return move_obj(grid, pos, dir);
        }
        return pos;
    }
    panic!("Invalid object");
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (grid, instructions) = input.split_once("\n\n").unwrap();

    let mut grid: CharGrid = grid.lines().map(|line| line.chars().collect()).collect();

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
            _ => robot_position,
        };
    }

    let mut total = 0;

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'O' {
                total += y * 100 + x;
            }
        }
    }

    println!("{}", total);
}
