use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let re = Regex::new(r"(do|don't|mul)\((\d+)?,?(\d+)?\)").unwrap();

    let (sum, _) = re
        .captures_iter(input.as_str())
        .fold((0, true), |(sum, is_enabled), c| {
            match c.get(1).unwrap().as_str() {
                "do" => (sum, true),
                "don't" => (sum, false),
                "mul" => {
                    if !is_enabled {
                        return (sum, is_enabled);
                    }

                    match (c.get(2), c.get(3)) {
                        (Some(left), Some(right)) => (
                            sum + left.as_str().parse::<u32>().unwrap()
                                * right.as_str().parse::<u32>().unwrap(),
                            is_enabled,
                        ),
                        _ => (sum, is_enabled),
                    }
                }
                _ => (sum, is_enabled),
            }
        });

    println!("{}", sum);
}
