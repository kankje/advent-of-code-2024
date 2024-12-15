use cgmath::{MetricSpace, Vector2};
use itertools::Itertools;
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

    for i in 0..10000 {
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

        let counts = robots.iter().fold(HashMap::new(), |mut counts, robot| {
            let counter = counts.entry(robot.pos).or_insert(0);
            *counter += 1;
            counts
        });

        let mut total_distance = 0;
        for combination in counts.keys().combinations(2) {
            let (a, b) = (combination[0], combination[1]);
            total_distance += a.distance2(*b);
        }

        if total_distance > 200000000 {
            continue;
        }

        println!("Iteration {}", i + 1);

        for y in 0..height {
            for x in 0..width {
                let count = counts.get(&Vector2::new(x, y)).unwrap_or(&0);
                if *count > 0 {
                    print!("{}", count);
                } else {
                    print!(".");
                }
            }
            println!();
        }

        return;
    }
}
