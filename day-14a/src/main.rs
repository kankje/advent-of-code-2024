use cgmath::Vector2;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

struct Robot {
    pos: Vector2<i32>,
    vel: Vector2<i32>,
}

fn main() {
    let lines = io::stdin().lock().lines();

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = lines
        .flat_map(|line| {
            let line = line.as_ref().unwrap();
            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [pos_x, pos_y, vel_x, vel_y])| Robot {
                    pos: Vector2::new(pos_x.parse::<i32>().unwrap(), pos_y.parse::<i32>().unwrap()),
                    vel: Vector2::new(vel_x.parse::<i32>().unwrap(), vel_y.parse::<i32>().unwrap()),
                })
                .collect::<Vec<Robot>>()
        })
        .collect();

    let width = 101;
    let height = 103;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.pos += robot.vel;
            if robot.pos.x < 0 {
                robot.pos.x += width;
            }
            if robot.pos.x >= width {
                robot.pos.x -= width;
            }
            if robot.pos.y < 0 {
                robot.pos.y += height;
            }
            if robot.pos.y >= height {
                robot.pos.y -= height;
            }
        }
    }

    let q_w = width / 2;
    let q_h = height / 2;

    let counts = robots.iter().fold(HashMap::new(), |mut counts, robot| {
        if robot.pos.x == q_w || robot.pos.y == q_h {
            return counts;
        }
        let q_x = robot.pos.x / (q_w + 1);
        let q_y = robot.pos.y / (q_h + 1);
        let counter = counts.entry(Vector2::new(q_x, q_y)).or_insert(0);
        *counter += 1;
        counts
    });

    let safety_factor: u64 = counts.values().product();

    println!("{}", safety_factor);
}
